# Measurement Tools

## Available tools

| Інструмент | Статус | Для чого |
| --- | --- | --- |
| Мультиметр | available | Перевірка `3V3`, `5V`, `GND`, continuity, рівнів кнопок |
| Логічний аналізатор USB | confirmed | GPIO timing, digital pulses, button bounce |
| Saleae Logic 2 | selected on macOS | Практичне ПЗ для логічного аналізатора |
| PicoScope 2206B MSO | available | Осцилограф/змішані сигнали, аналогові фронти, ADC input |

PulseView / `sigrok-cli` не вдалося стабільно використати з поточним логічним
аналізатором на macOS. Для практичної роботи обрано Saleae Logic 2.

## Default checks before firmware

- Мультиметром перевірити, що `3V3` справді близько `3.3V`.
- Якщо планується використовувати `5V` pin, перевірити його окремо на цій платі.
- Переконатися, що всі `GND` макета з'єднані з `GND` ESP32.
- Для LED перевірити наявність послідовного резистора.
- Для button module перевірити, що `OUT` не перевищує `3.3V`.

## Logic analyzer defaults

- GND аналізатора завжди з'єднати з GND ESP32.
- Для button/LED задач починати з sample rate `1-5 MS/s`.
- Для bounce імпульсів підняти sample rate, якщо імпульси вузькі.
- Не підключати канал логічного аналізатора до невідомого `5V` сигналу без
  перевірки допусків інструмента.

## Oscilloscope defaults

- Probe ground -> GND ESP32.
- Для ADC/LDR дивитися точку `GPIO4 / ADC1_CH3`.
- Для bounce homework дивитися `GPIO15`.
- Перед підключенням ground clip переконатися, що схема живиться від USB/ізольовано
  без небезпечної різниці потенціалів.
