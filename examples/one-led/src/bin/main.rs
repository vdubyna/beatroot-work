#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::Level;
use esp_hal::main;
use esp_hal::rmt::{PulseCode, Rmt, TxChannelConfig, TxChannelCreator};
use esp_hal::time::Rate;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.2.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // The onboard RGB LED on ESP32-S3 dev boards is commonly a WS2812-style LED on GPIO48.
    let rmt = match Rmt::new(peripherals.RMT, Rate::from_mhz(80)) {
        Ok(rmt) => rmt,
        Err(_) => loop {},
    };

    let tx_config = TxChannelConfig::default()
        .with_clk_divider(2)
        .with_idle_output_level(Level::Low)
        .with_idle_output(true);

    let mut led = match rmt.channel0.configure_tx(peripherals.GPIO48, tx_config) {
        Ok(channel) => channel,
        Err(_) => loop {},
    };

    let delay = Delay::new();
    let red = Rgb::new(16, 0, 0);
    let off = Rgb::new(0, 0, 0);
    let mut pulses = [PulseCode::end_marker(); WS2812_PULSE_COUNT];

    loop {
        write_ws2812(&mut pulses, red);
        led = transmit(led, &pulses);
        delay.delay_millis(500);

        write_ws2812(&mut pulses, off);
        led = transmit(led, &pulses);
        delay.delay_millis(500);
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0/examples
}

const WS2812_BITS_PER_LED: usize = 24;
const WS2812_PULSE_COUNT: usize = WS2812_BITS_PER_LED + 1;

const T0H: u16 = 14;
const T0L: u16 = 36;
const T1H: u16 = 28;
const T1L: u16 = 22;

#[derive(Clone, Copy)]
struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl Rgb {
    const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

fn write_ws2812(pulses: &mut [PulseCode; WS2812_PULSE_COUNT], color: Rgb) {
    let bytes = [color.green, color.red, color.blue];
    let mut index = 0;

    for byte in bytes {
        for bit in (0..8).rev() {
            pulses[index] = if byte & (1 << bit) != 0 {
                PulseCode::new(Level::High, T1H, Level::Low, T1L)
            } else {
                PulseCode::new(Level::High, T0H, Level::Low, T0L)
            };
            index += 1;
        }
    }

    pulses[index] = PulseCode::end_marker();
}

fn transmit<'channel>(
    channel: esp_hal::rmt::Channel<'channel, esp_hal::Blocking, esp_hal::rmt::Tx>,
    pulses: &[PulseCode; WS2812_PULSE_COUNT],
) -> esp_hal::rmt::Channel<'channel, esp_hal::Blocking, esp_hal::rmt::Tx> {
    let transaction = match channel.transmit(pulses) {
        Ok(transaction) => transaction,
        Err(_) => loop {},
    };

    match transaction.wait() {
        Ok(channel) => channel,
        Err(_) => loop {},
    }
}
