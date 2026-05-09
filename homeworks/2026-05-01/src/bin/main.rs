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

static BUTTON: Mutex<RefCell<Option<Input<'static>>>> = Mutex::new(RefCell::new(None));
static TOTAL_EDGES: AtomicU32 = AtomicU32::new(0);
static FALLING_EDGES: AtomicU32 = AtomicU32::new(0);
static RISING_EDGES: AtomicU32 = AtomicU32::new(0);
static LAST_EDGE_US: AtomicU32 = AtomicU32::new(0);
static LAST_LEVEL_LOW: AtomicBool = AtomicBool::new(false);

const TRIGGER_DEBOUNCE_US: u32 = 20_000;

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
    let mut button = Input::new(peripherals.GPIO15, input_config);
    let mut bounce_driver = Output::new(peripherals.GPIO16, Level::Low, OutputConfig::default());
    let initial_low = button.is_low();

    LAST_LEVEL_LOW.store(initial_low, Ordering::Relaxed);
    LAST_EDGE_US.store(now_micros_u32(), Ordering::Relaxed);
    button.listen(Event::AnyEdge);

    critical_section::with(|cs| {
        BUTTON.borrow_ref_mut(cs).replace(button);
    });

    println!("Boot: button bounce counter");
    println!("Board: ESP32-S3 YD-ESP32-23");
    println!("Trigger button: GPIO17 -> button -> GND");
    println!("Bounce driver: GPIO16 -> 4.7k -> 2N2222 base");
    println!("Measured signal: GPIO15 -> 10k -> 3V3, 2N2222 collector -> GPIO15, emitter -> GND");
    println!("Mode: GPIO17 triggers synthetic bounce on GPIO15; GPIO15 is counted without debounce");
    println!("Expected full press+release cycle: +5 irq_falling, +5 irq_rising");
    println!("irq_* = hardware interrupt counter; poll_* = 1 ms GPIO polling counter");
    print_state("boot", 0, 0, 0);

    let delay = Delay::new();
    let mut last_irq_total = TOTAL_EDGES.load(Ordering::Relaxed);
    let mut last_poll_low = button_is_low();
    let mut poll_total = 0u32;
    let mut poll_falling = 0u32;
    let mut poll_rising = 0u32;
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
                print_state("sim_press", poll_total, poll_falling, poll_rising);
            } else {
                println!("trigger=release, cycle={generated_cycles}, generating release bounce");
                generate_release_bounce(&mut bounce_driver, &delay);
                print_state("sim_release", poll_total, poll_falling, poll_rising);
            }
            last_report_us = now_micros_u32();
        }

        let current_poll_low = button_is_low();
        if current_poll_low != last_poll_low {
            poll_total += 1;
            if current_poll_low {
                poll_falling += 1;
            } else {
                poll_rising += 1;
            }
            last_poll_low = current_poll_low;
            print_state("poll", poll_total, poll_falling, poll_rising);
            last_report_us = now_micros_u32();
        }

        let irq_total = TOTAL_EDGES.load(Ordering::Relaxed);
        if irq_total != last_irq_total {
            print_state("irq", poll_total, poll_falling, poll_rising);
            last_irq_total = irq_total;
            last_report_us = now_micros_u32();
        }

        let now_us = now_micros_u32();
        if now_us.wrapping_sub(last_report_us) >= 1_000_000 {
            print_state("beat", poll_total, poll_falling, poll_rising);
            last_report_us = now_us;
        }

        delay.delay_millis(1);
    }
}

fn generate_press_bounce(driver: &mut Output<'_>, delay: &Delay) {
    // GPIO16 HIGH opens the NPN transistor, so GPIO15 becomes LOW.
    drive(driver, true, delay, 250);
    drive(driver, false, delay, 120);
    drive(driver, true, delay, 180);
    drive(driver, false, delay, 90);
    drive(driver, true, delay, 320);
}

fn generate_release_bounce(driver: &mut Output<'_>, delay: &Delay) {
    // End with transistor off, so GPIO15 returns to HIGH through the pull-up.
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
        let mut button_ref = BUTTON.borrow_ref_mut(cs);
        let Some(button) = button_ref.as_mut() else {
            return;
        };

        if !button.is_interrupt_set() {
            return;
        }

        button.clear_interrupt();

        let is_low = button.is_low();
        LAST_LEVEL_LOW.store(is_low, Ordering::Relaxed);
        LAST_EDGE_US.store(now_micros_u32(), Ordering::Relaxed);
        TOTAL_EDGES.fetch_add(1, Ordering::Relaxed);

        if is_low {
            FALLING_EDGES.fetch_add(1, Ordering::Relaxed);
        } else {
            RISING_EDGES.fetch_add(1, Ordering::Relaxed);
        }
    });
}

fn button_is_low() -> bool {
    critical_section::with(|cs| BUTTON.borrow_ref(cs).as_ref().is_some_and(Input::is_low))
}

fn print_state(tag: &str, poll_total: u32, poll_falling: u32, poll_rising: u32) {
    let irq_total = TOTAL_EDGES.load(Ordering::Relaxed);
    let irq_falling = FALLING_EDGES.load(Ordering::Relaxed);
    let irq_rising = RISING_EDGES.load(Ordering::Relaxed);
    let last_us = LAST_EDGE_US.load(Ordering::Relaxed);
    let level = if button_is_low() { "LOW" } else { "HIGH" };

    println!(
        "tag={tag}, irq_edges={irq_total}, irq_falling={irq_falling}, irq_rising={irq_rising}, poll_edges={poll_total}, poll_falling={poll_falling}, poll_rising={poll_rising}, last_irq_us={last_us}, level={level}"
    );
}

fn now_micros_u32() -> u32 {
    Instant::now().duration_since_epoch().as_micros() as u32
}
