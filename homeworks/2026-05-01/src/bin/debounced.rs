#![no_std]
#![no_main]

use core::cell::RefCell;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use critical_section::Mutex;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Event, Input, InputConfig, Io, Level, Output, OutputConfig, Pull};
use esp_hal::time::Instant;
use esp_hal::{handler, main};
use esp_println::println;

static BOUNCE_INPUT: Mutex<RefCell<Option<Input<'static>>>> = Mutex::new(RefCell::new(None));
static RAW_EDGES: AtomicU32 = AtomicU32::new(0);
static RAW_FALLING: AtomicU32 = AtomicU32::new(0);
static RAW_RISING: AtomicU32 = AtomicU32::new(0);
static DEBOUNCED_PRESSES: AtomicU32 = AtomicU32::new(0);
static DEBOUNCED_RELEASES: AtomicU32 = AtomicU32::new(0);
static LAST_RAW_EDGE_US: AtomicU32 = AtomicU32::new(0);
static LAST_ACCEPTED_US: AtomicU32 = AtomicU32::new(0);
static STABLE_LOW: AtomicBool = AtomicBool::new(false);

const TRIGGER_DEBOUNCE_US: u32 = 20_000;
const SIGNAL_DEBOUNCE_US: u32 = 5_000;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut io = Io::new(peripherals.IO_MUX);
    io.set_interrupt_handler(gpio_interrupt_handler);

    let input_config = InputConfig::default().with_pull(Pull::Up);
    let trigger = Input::new(peripherals.GPIO17, input_config);
    let mut bounce_input = Input::new(peripherals.GPIO15, input_config);
    let mut bounce_driver = Output::new(peripherals.GPIO16, Level::Low, OutputConfig::default());

    let initial_low = bounce_input.is_low();
    let now_us = now_micros_u32();
    STABLE_LOW.store(initial_low, Ordering::Relaxed);
    LAST_RAW_EDGE_US.store(now_us, Ordering::Relaxed);
    LAST_ACCEPTED_US.store(now_us, Ordering::Relaxed);
    bounce_input.listen(Event::AnyEdge);

    critical_section::with(|cs| {
        BOUNCE_INPUT.borrow_ref_mut(cs).replace(bounce_input);
    });

    println!("Boot: debounced button bounce counter");
    println!("Trigger button: GPIO17 -> button -> GND");
    println!("Bounce driver: GPIO16 -> 4.7k -> 2N2222 base");
    println!("Measured signal: GPIO15 -> 10k -> 3V3, 2N2222 collector -> GPIO15, emitter -> GND");
    println!("Raw counter sees every edge; debounced counter uses {SIGNAL_DEBOUNCE_US} us window");
    println!("Expected per full cycle: raw +5 falling/+5 rising, debounced +1 press/+1 release");
    print_state("boot", trigger.is_low());

    let delay = Delay::new();
    let mut last_raw_edges = RAW_EDGES.load(Ordering::Relaxed);
    let mut last_debounced_presses = DEBOUNCED_PRESSES.load(Ordering::Relaxed);
    let mut last_debounced_releases = DEBOUNCED_RELEASES.load(Ordering::Relaxed);
    let mut last_report_us = now_micros_u32();
    let mut trigger_raw_down = trigger.is_low();
    let mut trigger_stable_down = trigger_raw_down;
    let mut trigger_changed_us = last_report_us;
    let mut generated_cycles = 0u32;

    loop {
        let now_us = now_micros_u32();
        let current_trigger_raw_down = trigger.is_low();
        if current_trigger_raw_down != trigger_raw_down {
            trigger_raw_down = current_trigger_raw_down;
            trigger_changed_us = now_us;
        }

        if trigger_raw_down != trigger_stable_down
            && now_us.wrapping_sub(trigger_changed_us) >= TRIGGER_DEBOUNCE_US
        {
            trigger_stable_down = trigger_raw_down;
            if trigger_stable_down {
                generated_cycles += 1;
                println!("trigger=press, cycle={generated_cycles}, generating press bounce");
                generate_press_bounce(&mut bounce_driver, &delay);
                print_state("sim_press", trigger.is_low());
            } else {
                println!("trigger=release, cycle={generated_cycles}, generating release bounce");
                generate_release_bounce(&mut bounce_driver, &delay);
                print_state("sim_release", trigger.is_low());
            }
            last_report_us = now_micros_u32();
        }

        let raw_edges = RAW_EDGES.load(Ordering::Relaxed);
        let debounced_presses = DEBOUNCED_PRESSES.load(Ordering::Relaxed);
        let debounced_releases = DEBOUNCED_RELEASES.load(Ordering::Relaxed);
        if raw_edges != last_raw_edges
            || debounced_presses != last_debounced_presses
            || debounced_releases != last_debounced_releases
        {
            print_state("counter", trigger.is_low());
            last_raw_edges = raw_edges;
            last_debounced_presses = debounced_presses;
            last_debounced_releases = debounced_releases;
            last_report_us = now_micros_u32();
        }

        let now_us = now_micros_u32();
        if now_us.wrapping_sub(last_report_us) >= 1_000_000 {
            print_state("beat", trigger.is_low());
            last_report_us = now_us;
        }

        delay.delay_millis(1);
    }
}

fn generate_press_bounce(driver: &mut Output<'_>, delay: &Delay) {
    drive(driver, true, delay, 250);
    drive(driver, false, delay, 120);
    drive(driver, true, delay, 180);
    drive(driver, false, delay, 90);
    drive(driver, true, delay, 320);
}

fn generate_release_bounce(driver: &mut Output<'_>, delay: &Delay) {
    drive(driver, false, delay, 220);
    drive(driver, true, delay, 80);
    drive(driver, false, delay, 130);
    drive(driver, true, delay, 60);
    drive(driver, false, delay, 320);
}

fn drive(driver: &mut Output<'_>, transistor_on: bool, delay: &Delay, us: u32) {
    driver.set_level(if transistor_on { Level::High } else { Level::Low });
    delay.delay_micros(us);
}

#[handler]
fn gpio_interrupt_handler() {
    critical_section::with(|cs| {
        let mut input_ref = BOUNCE_INPUT.borrow_ref_mut(cs);
        let Some(input) = input_ref.as_mut() else {
            return;
        };

        if !input.is_interrupt_set() {
            return;
        }

        input.clear_interrupt();

        let now_us = now_micros_u32();
        let is_low = input.is_low();
        LAST_RAW_EDGE_US.store(now_us, Ordering::Relaxed);
        RAW_EDGES.fetch_add(1, Ordering::Relaxed);

        if is_low {
            RAW_FALLING.fetch_add(1, Ordering::Relaxed);
        } else {
            RAW_RISING.fetch_add(1, Ordering::Relaxed);
        }

        let stable_low = STABLE_LOW.load(Ordering::Relaxed);
        let last_accepted_us = LAST_ACCEPTED_US.load(Ordering::Relaxed);
        if is_low != stable_low
            && now_us.wrapping_sub(last_accepted_us) >= SIGNAL_DEBOUNCE_US
        {
            STABLE_LOW.store(is_low, Ordering::Relaxed);
            LAST_ACCEPTED_US.store(now_us, Ordering::Relaxed);

            if is_low {
                DEBOUNCED_PRESSES.fetch_add(1, Ordering::Relaxed);
            } else {
                DEBOUNCED_RELEASES.fetch_add(1, Ordering::Relaxed);
            }
        }
    });
}

fn print_state(tag: &str, trigger_low: bool) {
    let raw_edges = RAW_EDGES.load(Ordering::Relaxed);
    let raw_falling = RAW_FALLING.load(Ordering::Relaxed);
    let raw_rising = RAW_RISING.load(Ordering::Relaxed);
    let debounced_presses = DEBOUNCED_PRESSES.load(Ordering::Relaxed);
    let debounced_releases = DEBOUNCED_RELEASES.load(Ordering::Relaxed);
    let last_raw_us = LAST_RAW_EDGE_US.load(Ordering::Relaxed);
    let level = if input_is_low() { "LOW" } else { "HIGH" };
    let stable = if STABLE_LOW.load(Ordering::Relaxed) {
        "LOW pressed"
    } else {
        "HIGH released"
    };
    let trigger = if trigger_low { "LOW" } else { "HIGH" };

    println!(
        "tag={tag}, trigger={trigger}, raw_edges={raw_edges}, raw_falling={raw_falling}, raw_rising={raw_rising}, debounced_presses={debounced_presses}, debounced_releases={debounced_releases}, last_raw_us={last_raw_us}, level={level}, stable={stable}"
    );
}

fn input_is_low() -> bool {
    critical_section::with(|cs| {
        BOUNCE_INPUT
            .borrow_ref(cs)
            .as_ref()
            .is_some_and(Input::is_low)
    })
}

fn now_micros_u32() -> u32 {
    Instant::now().duration_since_epoch().as_micros() as u32
}
