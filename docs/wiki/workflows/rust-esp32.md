# Rust ESP32 Workflow

## Create a new homework

1. Copy the structure of the newest Rust homework.
2. Update `Cargo.toml` package name and `[[bin]]` names.
3. Keep target/toolchain config for `xtensa-esp32s3-none-elf`.
4. Set runner to the currently observed serial port, usually
   `/dev/cu.usbserial-A5069RR4`.
5. Write `README.md` with parts, wiring, run commands, expected output, and
   measurement checks.
6. Update `docs/wiki/projects/index.md` and any hardware/parts notes.

## Pick GPIO

- Prefer pins already validated in previous homeworks when the electrical role
  matches.
- For ADC on ESP32-S3 use `GPIO1-GPIO10` / ADC1 first.
- For voltage measurements with `esp-hal`, enable ADC pins with calibration,
  for example `enable_pin_with_cal::<_, AdcCalCurve<ADC1<'_>>>(...)`.
- Avoid `GPIO0` unless intentionally handling boot mode.
- Avoid `GPIO19/GPIO20` unless working with native USB.
- Avoid `GPIO35-GPIO37` until board-specific behavior is verified.
- Keep GPIO at `3.3V` logic.

## Build and run

```sh
cargo build
cargo run --bin <bin-name>
```

Run inside the project directory. `cargo run` flashes via `espflash` and opens
monitor when `.cargo/config.toml` is configured.

## README checklist

Each homework README should include:

- Goal in one paragraph.
- Parts table.
- Wiring table.
- Pin safety notes.
- Commands for build/run.
- Expected serial output or visual result.
- Hardware verification step using multimeter, logic analyzer, or oscilloscope
  when useful.

## When code and hardware disagree

- First verify wiring and power with measurement tools.
- Then verify `.cargo/config.toml` port.
- Then add temporary serial prints or a small scanner bin.
- Record the confirmed fact in `docs/wiki/` if it will matter later.
