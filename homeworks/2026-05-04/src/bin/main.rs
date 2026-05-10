#![no_std]
#![no_main]

use esp_hal::analog::adc::{Adc, AdcCalCurve, AdcConfig, Attenuation};
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::peripherals::ADC1;
use esp_println::println;

const SAMPLE_DELAY_MS: u32 = 100;
const YELLOW_MIN_MV: u16 = 2_000;
const RED_MIN_MV: u16 = 2_600;

// ADC attenuation sets the measurable voltage range inside the ESP32-S3 ADC.
// With _11dB the ADC can measure close to the 0-3.1V range we expect from this
// LDR divider. Lower attenuation gives finer low-voltage detail, but saturates
// earlier when the input voltage rises.
const ATTENUATION: Attenuation = Attenuation::_11dB;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut adc1_config = AdcConfig::new();

    // Use the calibrated ADC path. On ESP32-S3 an uncalibrated ADC read has a
    // large offset: during debugging a pin tied to GND looked like ~1.6-1.8V.
    // AdcCalCurve applies the chip calibration and makes GND read as 0 mV.
    let mut ldr_pin =
        adc1_config.enable_pin_with_cal::<_, AdcCalCurve<ADC1<'_>>>(peripherals.GPIO4, ATTENUATION);
    let mut adc1 = Adc::new(peripherals.ADC1, adc1_config);

    // LEDs are active-high: GPIO High -> current flows through the LED/resistor
    // to GND. Wiring used in this homework: GPIO15 green, GPIO16 yellow,
    // GPIO17 red.
    let mut green = Output::new(peripherals.GPIO15, Level::Low, OutputConfig::default());
    let mut yellow = Output::new(peripherals.GPIO16, Level::Low, OutputConfig::default());
    let mut red = Output::new(peripherals.GPIO17, Level::Low, OutputConfig::default());

    let delay = Delay::new();

    println!("Boot: LDR ADC voltage homework");
    println!("Board: ESP32-S3 YD-ESP32-23");
    println!("LDR divider output: GPIO4 / ADC1_CH3");
    println!("Circuit: 3V3 -> LDR -> GPIO4 -> 10k -> GND");
    println!("LEDs: green=GPIO15, yellow=GPIO16, red=GPIO17");
    println!("Thresholds: green always shows low level, yellow >= {YELLOW_MIN_MV} mV, red >= {RED_MIN_MV} mV");
    println!("Sampling period: {SAMPLE_DELAY_MS} ms");
    println!("ADC calibration: AdcCalCurve");
    println!("time_ms,u_calibrated_mv,level");

    let mut time_ms = 0u32;

    loop {
        // Because the pin was enabled with AdcCalCurve, read_blocking returns
        // calibrated millivolts instead of raw 12-bit ADC counts.
        let u_calibrated_mv = adc1.read_blocking(&mut ldr_pin);
        let level = show_light_level(u_calibrated_mv, &mut green, &mut yellow, &mut red);
        println!("{time_ms},{u_calibrated_mv},{level}");

        delay.delay_millis(SAMPLE_DELAY_MS);
        time_ms = time_ms.wrapping_add(SAMPLE_DELAY_MS);
    }
}

fn show_light_level(
    u_mv: u16,
    green: &mut Output<'_>,
    yellow: &mut Output<'_>,
    red: &mut Output<'_>,
) -> &'static str {
    // Bar-style indicator: higher brightness keeps lower levels on.
    // Low: green; medium: green + yellow; bright: green + yellow + red.
    green.set_high();

    if u_mv >= RED_MIN_MV {
        yellow.set_high();
        red.set_high();
        "bright"
    } else if u_mv >= YELLOW_MIN_MV {
        yellow.set_high();
        red.set_low();
        "medium"
    } else {
        green.set_high();
        yellow.set_low();
        red.set_low();
        "low"
    }
}
