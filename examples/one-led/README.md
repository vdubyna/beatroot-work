# ESP32-S3 Rust examples

Rust приклади для нашої ESP32-S3 плати.

## Плата

- Плата: `YD-ESP32-23 2022-V1.3`
- Чип/модуль: ESP32-S3, модуль з маркуванням `S3-N16R8`
- Порт для прошивки: `COM`, зараз macOS бачить його як `/dev/cu.usbserial-A5069RR4`

## Button right

Прошивка `button-right` читає кнопку на `GPIO1`, використовує внутрішній pull-up і рахує натискання через GPIO interrupt.

Підключення:

| Кнопка | Плата |
| --- | --- |
| одна ніжка | пін `1` / `GPIO1` |
| друга ніжка | `GND` |

Запуск і прошивка:

```sh
cargo run --bin button-right
```

Очікуваний serial output:

```text
Boot...
BUTTON_RIGHT: GPIO1 -> GND, internal pull-up enabled
Press the button; RIGHT counter should increase once per press
RIGHT: 0 (HIGH idle)
RIGHT: 1
RIGHT: 1 (HIGH idle)
```

Коли кнопка не натиснута, має бути `HIGH idle`. Коли тримаєш кнопку, має бути `LOW pressed`. Кожне нормальне натискання має збільшувати `RIGHT` на `1`.

Якщо monitor постійно показує:

```text
RIGHT: 0 (LOW pressed)
```

то `GPIO1` фізично притягнутий до `GND`. Перевір:

- кнопку не треба тримати натиснутою під час старту;
- одна ніжка кнопки має йти на `GPIO1`, друга на `GND`;
- якщо це 4-ніжкова tactile-кнопка на breadboard, дроти мають бути на протилежних сторонах кнопки, а не на двох ніжках однієї внутрішньо з'єднаної сторони;
- спробуй повернути кнопку на 90 градусів або переставити дріт на іншу ніжку.

## One LED

Приклад вважає, що вбудований RGB LED підключений як WS2812-style LED на `GPIO48`.
Якщо прошивка запускається, але LED не блимає, найімовірніше треба змінити `peripherals.GPIO48` у `src/bin/main.rs` на правильний GPIO для цієї плати.

Запуск і прошивка:

```sh
cargo run --bin one-led
```
