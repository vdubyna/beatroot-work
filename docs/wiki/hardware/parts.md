# Parts Inventory

Статуси:

- `confirmed` - було підтверджено фото, вимірюванням або запуском.
- `used in project docs` - згадується в README домашок як потрібна/використана
  деталь.
- `to verify` - треба підтвердити кількість або конкретний номінал.

## Boards and modules

| Деталь | Кількість | Статус | Нотатка |
| --- | ---: | --- | --- |
| ESP32-S3 dev board `YD-ESP32-23` | 1 | confirmed | Основна плата |
| Button module `VCC/OUT/GND` | 1+ | confirmed | Active-low у наших підключеннях: idle `HIGH`, pressed `LOW` |
| Breadboard | 1 | used in project docs | Для макетів LED, кнопок, LDR |

## Passive and simple active parts

| Деталь | Номінал / тип | Статус | Де використано |
| --- | --- | --- | --- |
| LED червоний | 3/5 mm | used in project docs | `homeworks/2026-04-27`, `2026-04-29` |
| LED синій/зелений | 3/5 mm | used in project docs | `homeworks/2026-04-27`, `2026-04-29` |
| Резистори для LED | `220-330 Ohm` | used in project docs | Послідовно з LED |
| Pull-up resistor | `10 kOhm` | used in project docs | Bounce, LDR divider |
| Base resistor | `4.7 kOhm` | used in project docs | 2N2222 base |
| LDR / фоторезистор | опір залежить від освітлення | used in project docs | `homeworks/2026-05-04` |
| Конденсатор | `0.1 uF` optional | used in project docs | Фільтр ADC input |
| NPN transistor | `2N2222` | used in project docs | Штучний bounce generator |
| Jumper wires | male-male / mixed | used in project docs | Усі макети |

## Safe defaults

- Для button module: `VCC -> 3V3`, `OUT -> GPIO`, `GND -> GND`, у коді
  `Pull::Up`, натиснуто = `LOW`.
- Для LED: `GPIO -> resistor -> LED anode`, `LED cathode -> GND`.
- Для LDR divider: `3V3 -> LDR -> ADC point -> 10 kOhm -> GND`.
- Для невідомого модуля спочатку перевірити мультиметром живлення і рівень
  `OUT`, перш ніж підключати до GPIO.
