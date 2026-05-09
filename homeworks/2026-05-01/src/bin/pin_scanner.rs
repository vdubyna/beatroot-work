#![no_std]
#![no_main]

use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Input, InputConfig, Pull};
use esp_hal::main;
use esp_println::println;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let input_config = InputConfig::default().with_pull(Pull::Up);

    let gpio4 = Input::new(peripherals.GPIO4, input_config);
    let gpio5 = Input::new(peripherals.GPIO5, input_config);
    let gpio6 = Input::new(peripherals.GPIO6, input_config);
    let gpio7 = Input::new(peripherals.GPIO7, input_config);
    let gpio8 = Input::new(peripherals.GPIO8, input_config);
    let gpio9 = Input::new(peripherals.GPIO9, input_config);
    let gpio10 = Input::new(peripherals.GPIO10, input_config);
    let gpio11 = Input::new(peripherals.GPIO11, input_config);
    let gpio12 = Input::new(peripherals.GPIO12, input_config);
    let gpio13 = Input::new(peripherals.GPIO13, input_config);
    let gpio14 = Input::new(peripherals.GPIO14, input_config);
    let gpio15 = Input::new(peripherals.GPIO15, input_config);
    let gpio16 = Input::new(peripherals.GPIO16, input_config);
    let gpio17 = Input::new(peripherals.GPIO17, input_config);
    let gpio18 = Input::new(peripherals.GPIO18, input_config);

    println!("Boot: GPIO pin scanner");
    println!("All scanned pins use internal pull-up.");
    println!("Press the button and look for a pin that changes H -> L.");

    let delay = Delay::new();
    let mut last_mask = read_mask(
        &gpio4, &gpio5, &gpio6, &gpio7, &gpio8, &gpio9, &gpio10, &gpio11, &gpio12, &gpio13,
        &gpio14, &gpio15, &gpio16, &gpio17, &gpio18,
    );
    print_mask("boot", last_mask);

    loop {
        let mask = read_mask(
            &gpio4, &gpio5, &gpio6, &gpio7, &gpio8, &gpio9, &gpio10, &gpio11, &gpio12,
            &gpio13, &gpio14, &gpio15, &gpio16, &gpio17, &gpio18,
        );

        if mask != last_mask {
            print_mask("change", mask);
            last_mask = mask;
        }

        delay.delay_millis(5);
    }
}

#[allow(clippy::too_many_arguments)]
fn read_mask(
    gpio4: &Input<'_>,
    gpio5: &Input<'_>,
    gpio6: &Input<'_>,
    gpio7: &Input<'_>,
    gpio8: &Input<'_>,
    gpio9: &Input<'_>,
    gpio10: &Input<'_>,
    gpio11: &Input<'_>,
    gpio12: &Input<'_>,
    gpio13: &Input<'_>,
    gpio14: &Input<'_>,
    gpio15: &Input<'_>,
    gpio16: &Input<'_>,
    gpio17: &Input<'_>,
    gpio18: &Input<'_>,
) -> u32 {
    let pins = [
        gpio4.is_high(),
        gpio5.is_high(),
        gpio6.is_high(),
        gpio7.is_high(),
        gpio8.is_high(),
        gpio9.is_high(),
        gpio10.is_high(),
        gpio11.is_high(),
        gpio12.is_high(),
        gpio13.is_high(),
        gpio14.is_high(),
        gpio15.is_high(),
        gpio16.is_high(),
        gpio17.is_high(),
        gpio18.is_high(),
    ];

    let mut mask = 0u32;
    for (index, is_high) in pins.iter().enumerate() {
        if *is_high {
            mask |= 1 << index;
        }
    }

    mask
}

fn print_mask(tag: &str, mask: u32) {
    println!(
        "tag={tag}, GPIO4={}, GPIO5={}, GPIO6={}, GPIO7={}, GPIO8={}, GPIO9={}, GPIO10={}, GPIO11={}, GPIO12={}, GPIO13={}, GPIO14={}, GPIO15={}, GPIO16={}, GPIO17={}, GPIO18={}",
        level(mask, 0),
        level(mask, 1),
        level(mask, 2),
        level(mask, 3),
        level(mask, 4),
        level(mask, 5),
        level(mask, 6),
        level(mask, 7),
        level(mask, 8),
        level(mask, 9),
        level(mask, 10),
        level(mask, 11),
        level(mask, 12),
        level(mask, 13),
        level(mask, 14),
    );
}

fn level(mask: u32, index: u32) -> &'static str {
    if mask & (1 << index) != 0 { "H" } else { "L" }
}
