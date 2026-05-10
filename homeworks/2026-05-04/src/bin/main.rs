#![no_std]
#![no_main]

use esp_hal::analog::adc::{Adc, AdcCalCurve, AdcConfig, Attenuation};
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::peripherals::ADC1;
use esp_println::println;

const SAMPLE_DELAY_MS: u32 = 100;

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

    let delay = Delay::new();

    println!("Boot: LDR ADC voltage homework");
    println!("Board: ESP32-S3 YD-ESP32-23");
    println!("LDR divider output: GPIO4 / ADC1_CH3");
    println!("Circuit: 3V3 -> LDR -> GPIO4 -> 10k -> GND");
    println!("Sampling period: {SAMPLE_DELAY_MS} ms");
    println!("ADC calibration: AdcCalCurve");
    println!("time_ms,u_calibrated_mv");

    let mut time_ms = 0u32;

    loop {
        // Because the pin was enabled with AdcCalCurve, read_blocking returns
        // calibrated millivolts instead of raw 12-bit ADC counts.
        let u_calibrated_mv = adc1.read_blocking(&mut ldr_pin);
        println!("{time_ms},{u_calibrated_mv}");

        delay.delay_millis(SAMPLE_DELAY_MS);
        time_ms = time_ms.wrapping_add(SAMPLE_DELAY_MS);
    }
}
