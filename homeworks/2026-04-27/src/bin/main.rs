#![no_std]
#![no_main]
// ESP32 firmware is not a normal desktop program:
// - `no_std` means we do not use the full Rust standard library, because there
//   is no operating system on the microcontroller.
// - `no_main` means the usual `fn main()` startup from desktop Rust is replaced
//   by the ESP HAL startup code and the `#[main]` function below.
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

// `CpuClock` lets us choose the CPU speed.
use esp_hal::clock::CpuClock;
// `Delay` gives simple blocking pauses in milliseconds.
use esp_hal::delay::Delay;
// `Output` controls a GPIO pin as digital output.
// `Level::High` means 3.3V on the pin, `Level::Low` means 0V/GND.
use esp_hal::gpio::{Level, Output, OutputConfig};
// `#[main]` is the ESP HAL entry point macro for firmware.
use esp_hal::main;

// How long one LED flash stays ON.
const FLASH_MS: u32 = 90;
// Pause between short flashes of the same color.
const GAP_MS: u32 = 70;
// Pause after red series before blue series, and after blue before red.
const SWITCH_GAP_MS: u32 = 180;
// Number of short flashes for one color before switching to the other color.
const BURSTS_PER_COLOR: u8 = 3;

// In embedded firmware we must provide our own panic handler.
// If something unrecoverable happens, the board stays in an infinite loop.
// For this simple homework we do not print panic messages to serial.
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates an application descriptor required by the ESP-IDF bootloader.
// Without it the bootloader may refuse to start our firmware image.
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // Create default chip configuration and run the CPU at the maximum
    // frequency supported by this board/chip.
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());

    // Initialize ESP32 peripherals: GPIO, timers, clocks, etc.
    // After this call Rust gives us strongly typed access to concrete pins like
    // `peripherals.GPIO4` and `peripherals.GPIO5`.
    let peripherals = esp_hal::init(config);

    // Default output config means push-pull digital output with no pull-up or
    // pull-down resistor. This is the usual mode for driving an LED through an
    // external resistor.
    let output_config = OutputConfig::default();

    // Red LED:
    // GPIO4 -> resistor -> long LED leg/anode, short LED leg/cathode -> GND.
    // Initial level is LOW, so the LED starts turned off.
    let mut red = Output::new(peripherals.GPIO4, Level::Low, output_config);

    // Blue LED:
    // GPIO5 -> resistor -> long LED leg/anode, short LED leg/cathode -> GND.
    // Initial level is also LOW, so both LEDs are off when firmware starts.
    let mut blue = Output::new(peripherals.GPIO5, Level::Low, output_config);

    // Simple blocking delay provider. While `delay_millis` is running, the CPU
    // waits and does not do other work. For this homework that is perfectly OK.
    let delay = Delay::new();

    // Firmware normally never exits. This loop is the whole program:
    // red flashes, short pause, blue flashes, short pause, repeat forever.
    loop {
        // Flash red while blue is forced off.
        flash_color(&mut red, &mut blue, &delay);
        delay.delay_millis(SWITCH_GAP_MS);

        // Flash blue while red is forced off.
        flash_color(&mut blue, &mut red, &delay);
        delay.delay_millis(SWITCH_GAP_MS);
    }
}

// Flash one selected LED several times.
//
// `active` is the LED that should blink now.
// `inactive` is the other LED; we explicitly turn it off to guarantee that
// red and blue never shine at the same time.
// `delay` is shared because it has no internal state that we need to mutate.
fn flash_color(active: &mut Output<'_>, inactive: &mut Output<'_>, delay: &Delay) {
    // Our wiring is active-high:
    // - HIGH = GPIO outputs 3.3V, current flows through resistor and LED to GND.
    // - LOW = GPIO outputs 0V, no useful voltage difference, LED is off.
    inactive.set_low();

    // Example with BURSTS_PER_COLOR = 3:
    // ON, pause, ON, pause, ON, pause.
    for _ in 0..BURSTS_PER_COLOR {
        // Turn selected LED on.
        active.set_high();
        delay.delay_millis(FLASH_MS);

        // Turn selected LED off.
        active.set_low();
        delay.delay_millis(GAP_MS);
    }
}
