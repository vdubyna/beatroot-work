# Projects Index

## Directory map

```text
examples/
  one-led/              Rust examples for the current ESP32-S3 board
homeworks/
  2026-04-27/           police flasher
  2026-04-29/           button-controlled LED modes
  2026-05-01/           button bounce and debounce
  2026-05-04/           LDR, ADC, voltage
lessons/
  2026-05-01/           Arduino/C++ explanation notes
docs/
  wiki/                 LLM-readable project knowledge
  reference/            local PDFs/images
```

## Rust projects

| Path | Bin(s) | Purpose | Pins |
| --- | --- | --- | --- |
| `examples/one-led` | `one-led` | Onboard RGB LED blink via RMT/WS2812-style output | `GPIO48` |
| `examples/one-led` | `button-right` | Read simple button with interrupt | `GPIO1 -> GND` |
| `homeworks/2026-04-27` | `police-flasher` | Red/blue LED police-style blinking | `GPIO4`, `GPIO5` |
| `homeworks/2026-04-29` | `button-led-modes` | Buttons switch LED speed modes | LED `GPIO4/GPIO5`, buttons `GPIO6/GPIO7` |
| `homeworks/2026-05-01` | `button-bounce-counter` | Count raw bounce edges | trigger `GPIO17`, drive `GPIO16`, input `GPIO15` |
| `homeworks/2026-05-01` | `button-bounce-debounced` | Same signal with software debounce | trigger `GPIO17`, drive `GPIO16`, input `GPIO15` |
| `homeworks/2026-05-01` | `pin-scanner` | GPIO diagnostic scanner | project-specific |
| `homeworks/2026-05-04` | `ldr-adc-voltage` | LDR divider with green/yellow/red LED brightness indicator | LDR `GPIO4 / ADC1_CH3`; LEDs `GPIO15-GPIO17` |

## Lessons

| Path | Topic |
| --- | --- |
| `lessons/2026-05-01/arduino-button-delay.md` | Arduino-style interrupt/debounce explanation, mapped to our board |

## Notes for future agents

- Treat each `homeworks/YYYY-MM-DD` as an independent Cargo project.
- Do not assume one homework's GPIO is free in another physical setup; check the
  README for the active wiring.
- If a project uses `GPIO4` both for LED and ADC in different homeworks, that is
  not a conflict unless the hardware is wired at the same time.
- For ESP32-S3 ADC voltage readings with `esp-hal`, prefer
  `enable_pin_with_cal::<_, AdcCalCurve<ADC1<'_>>>(...)`; uncalibrated ADC can
  report a large offset even when the pin is tied to `GND`.
- Update this index when adding a new `src/bin/*.rs` or changing pins.
