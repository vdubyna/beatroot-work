# Development Tools

## Rust ESP32 stack

У цьому репозиторії домашні роботи для мікроконтролерів робимо на Rust.

Основні компоненти:

- `cargo` - build/run.
- `esp-hal` - hardware abstraction для ESP32-S3.
- `esp-bootloader-esp-idf` - bootloader stack у поточних проєктах.
- `esp-println` - serial logging.
- `espflash` - flashing + serial monitor.

## Local runner pattern

Типовий `.cargo/config.toml`:

```toml
[target.xtensa-esp32s3-none-elf]
runner = "espflash flash --monitor --chip esp32s3 --port /dev/cu.usbserial-A5069RR4 --non-interactive --skip-update-check"
```

Для `homeworks/2026-04-27` runner ще використовує `/dev/cu.usbmodem2101`. Якщо
плата підключена через `COM`/USB-UART і macOS бачить інший device, оновити
runner у конкретному проєкті.

## Useful commands

```sh
cargo build
cargo run --bin one-led
cargo run --bin button-right
cargo run --bin police-flasher
cargo run --bin button-led-modes
cargo run --bin button-bounce-counter
cargo run --bin button-bounce-debounced
cargo run --bin ldr-adc-voltage
cargo run --bin ldr-adc-sweep
```

Запускати команди з директорії відповідного Cargo-проєкту.

## New homework structure

Для нових домашок повторювати структуру:

```text
homeworks/YYYY-MM-DD/
  .cargo/config.toml
  .clippy.toml
  .gitignore
  Cargo.toml
  build.rs
  rust-toolchain.toml
  src/bin/*.rs
  README.md
```

Підбирати GPIO за `docs/wiki/hardware/yd-esp32-23.md`, а не за generic ESP32
прикладами з інтернету.
