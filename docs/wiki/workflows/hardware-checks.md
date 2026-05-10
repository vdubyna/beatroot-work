# Hardware Check Workflows

## Before connecting a new module

1. Read markings on the module.
2. Identify `VCC`, `GND`, signal pins.
3. Check whether signal output is `3.3V` or `5V`.
4. Power from `3V3` by default for simple sensors/buttons connected to GPIO.
5. Measure idle signal level before connecting to an ESP32 GPIO if unsure.

## Button module check

Expected module pins: `VCC`, `OUT`, `GND`.

```text
VCC -> 3V3
OUT -> GPIO
GND -> GND
```

Expected behavior in current notes:

- idle = `HIGH`;
- pressed = `LOW`;
- Rust GPIO config should use pull-up or be compatible with active-low logic.

Use a multimeter or logic analyzer on `OUT` to confirm before relying on code.

## LED check

```text
GPIO -> 220-330 Ohm resistor -> LED anode
LED cathode -> GND
```

If LED does not light:

- flip LED polarity;
- check resistor is in series, not parallel;
- measure voltage on GPIO while firmware says LED is on;
- try a slow blink before debugging timing logic.

## ADC / LDR check

Preferred ESP32-S3 ADC pins: `GPIO1-GPIO10`.

Current LDR divider:

```text
3V3 ---- LDR ----+---- GPIO4 / ADC1_CH3
                 |
               10 kOhm
                 |
                GND
```

Checks:

- Measure voltage at the divider midpoint with a multimeter.
- Keep voltage between `0V` and `3.3V`.
- Use oscilloscope if readings jump quickly.
- Add `0.1 uF` from ADC point to GND if the signal is noisy.

## PSRAM check

The board marking `N16R8` suggests `16MB` flash and `8MB` PSRAM. Flash is
confirmed. PSRAM still needs a firmware-level check.

Future check idea:

- Add a tiny Rust bin that initializes/prints PSRAM availability if supported by
  the current stack.
- Record result in `docs/wiki/hardware/yd-esp32-23.md`.
