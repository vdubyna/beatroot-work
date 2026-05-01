#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use core::cell::RefCell;
use core::sync::atomic::{AtomicU32, Ordering};

use critical_section::Mutex;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Event, Input, InputConfig, Io, Pull};
use esp_hal::time::Instant;
use esp_hal::{handler, main};
use esp_println::println;

const DEBOUNCE_MS: u64 = 100;

static BUTTON_RIGHT: Mutex<RefCell<Option<Input<'static>>>> = Mutex::new(RefCell::new(None));
static COUNTER_RIGHT: AtomicU32 = AtomicU32::new(0);
static LAST_ACCEPTED_MS: AtomicU32 = AtomicU32::new(0);

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut io = Io::new(peripherals.IO_MUX);
    io.set_interrupt_handler(gpio_interrupt_handler);

    let button_config = InputConfig::default().with_pull(Pull::Up);
    let mut button = Input::new(peripherals.GPIO1, button_config);

    LAST_ACCEPTED_MS.store(now_millis_u32(), Ordering::Relaxed);
    button.listen(Event::FallingEdge);

    critical_section::with(|cs| {
        BUTTON_RIGHT.borrow_ref_mut(cs).replace(button);
    });

    println!("Boot...");
    println!("BUTTON_RIGHT: GPIO1 -> GND, internal pull-up enabled");
    println!("Press the button; RIGHT counter should increase once per press");

    let delay = Delay::new();
    let mut last_reported_count = COUNTER_RIGHT.load(Ordering::Relaxed);

    loop {
        let count = COUNTER_RIGHT.load(Ordering::Relaxed);
        let pressed = button_is_pressed();

        if count != last_reported_count {
            println!("RIGHT: {count}");
            last_reported_count = count;
        } else {
            let level = if pressed { "LOW pressed" } else { "HIGH idle" };
            println!("RIGHT: {count} ({level})");
        }

        delay.delay_millis(250);
    }
}

#[handler]
fn gpio_interrupt_handler() {
    critical_section::with(|cs| {
        let mut button_ref = BUTTON_RIGHT.borrow_ref_mut(cs);
        let Some(button) = button_ref.as_mut() else {
            return;
        };

        if !button.is_interrupt_set() {
            return;
        }

        button.clear_interrupt();

        let now_ms = now_millis_u32();
        let last_ms = LAST_ACCEPTED_MS.load(Ordering::Relaxed);

        if now_ms.wrapping_sub(last_ms) > DEBOUNCE_MS as u32 {
            LAST_ACCEPTED_MS.store(now_ms, Ordering::Relaxed);
            COUNTER_RIGHT.fetch_add(1, Ordering::Relaxed);
        }
    });
}

fn button_is_pressed() -> bool {
    critical_section::with(|cs| {
        BUTTON_RIGHT
            .borrow_ref(cs)
            .as_ref()
            .is_some_and(Input::is_low)
    })
}

fn now_millis_u32() -> u32 {
    Instant::now().duration_since_epoch().as_millis() as u32
}
