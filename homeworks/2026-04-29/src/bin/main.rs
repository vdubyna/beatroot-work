#![no_std]
#![no_main]

use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::main;
use esp_println::println;

const FAST_MS: u32 = 100;
const SLOW_MS: u32 = 600;
const POLL_MS: u32 = 10;
const DEBOUNCE_MS: u32 = 120;
const LED_STEPS: [(bool, bool); 4] = [(true, false), (false, false), (false, true), (false, false)];

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut red = Output::new(peripherals.GPIO4, Level::Low, OutputConfig::default());
    let mut blue = Output::new(peripherals.GPIO5, Level::Low, OutputConfig::default());

    let button_config = InputConfig::default().with_pull(Pull::Up);
    let fast_button = Input::new(peripherals.GPIO6, button_config);
    let slow_button = Input::new(peripherals.GPIO7, button_config);

    let delay = Delay::new();
    let mut blink_ms = SLOW_MS;
    let mut was_down = (fast_button.is_low(), slow_button.is_low());

    println!("LEDs: GPIO4, GPIO5");
    println!("FAST button: GPIO6, SLOW button: GPIO7");
    print_speed(blink_ms);

    loop {
        for (red_on, blue_on) in LED_STEPS {
            red.set_level(if red_on { Level::High } else { Level::Low });
            blue.set_level(if blue_on { Level::High } else { Level::Low });
            blink_ms = wait_buttons(blink_ms, &fast_button, &slow_button, &mut was_down, &delay);
        }
    }
}

fn wait_buttons(
    blink_ms: u32,
    fast_button: &Input<'_>,
    slow_button: &Input<'_>,
    was_down: &mut (bool, bool),
    delay: &Delay,
) -> u32 {
    let mut waited_ms = 0;

    while waited_ms < blink_ms {
        let step_ms = (blink_ms - waited_ms).min(POLL_MS);
        delay.delay_millis(step_ms);
        waited_ms += step_ms;

        let fast_down = fast_button.is_low();
        let slow_down = slow_button.is_low();
        let new_blink_ms = if fast_down && !was_down.0 {
            Some(FAST_MS)
        } else if slow_down && !was_down.1 {
            Some(SLOW_MS)
        } else {
            None
        };

        *was_down = (fast_down, slow_down);

        if let Some(new_blink_ms) = new_blink_ms {
            if new_blink_ms != blink_ms {
                print_speed(new_blink_ms);
            }
            delay.delay_millis(DEBOUNCE_MS);
            return new_blink_ms;
        }
    }

    blink_ms
}

fn print_speed(blink_ms: u32) {
    println!("Blink delay: {blink_ms} ms");
}
