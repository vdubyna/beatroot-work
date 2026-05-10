# ESP32-S3 YD-ESP32-23

## Identity

| Поле | Значення | Статус |
| --- | --- | --- |
| Board marking | `YD-ESP32-23`, `2022-V1.3` | photo |
| Module marking | `S3-N16R8 WIFI+BT MODULE` | photo |
| Expected module | `ESP32-S3-WROOM-1-N16R8` | datasheet + community source |
| Chip | `esp32s3`, revision `v0.2` | confirmed on board |
| Crystal | `40 MHz` | confirmed on board |
| Flash | `16MB` | confirmed on board |
| PSRAM | `8MB` expected from `N16R8` | assumption, verify |
| Wireless | Wi-Fi + BLE | confirmed by chip/features |
| MAC seen | `3c:0f:02:da:21:d8` | confirmed on board |

## USB and flashing

| Порт / вузол | Призначення | Нотатка |
| --- | --- | --- |
| `COM` / USB-UART | Прошивка і serial monitor | Основний шлях для `cargo run` |
| `/dev/cu.usbserial-A5069RR4` | Поточний macOS serial device | Використовується в більшості `.cargo/config.toml` |
| `USB` / `OTG-USB` | Native USB / USB OTG ESP32-S3 | Перевіряти окремо, не змішувати зі стартовою прошивкою |

Важлива різниця джерел: наша macOS-перевірка бачила `FT232R USB UART`, а
community-документи для YD-ESP32-S3 часто згадують `CH343P`. Тому USB-UART chip
може залежати від ревізії/клону. Для практики в цьому репозиторії важливіший
фактичний serial device, який бачить macOS.

## Pin notes

| Пін | Використання / ризик |
| --- | --- |
| `GPIO0` | Boot mode; кнопка `BOOT` |
| `GPIO1-GPIO10` | ADC1 на ESP32-S3 |
| `GPIO4` | Використовується для LED і LDR/ADC у домашках; `ADC1_CH3` |
| `GPIO5` | Зовнішній LED у домашках |
| `GPIO6`, `GPIO7` | Кнопки-модулі `FAST`/`SLOW` |
| `GPIO15` | Вхід для вимірювання штучного bounce |
| `GPIO16` | Керує transistor bounce generator |
| `GPIO17` | Реальна кнопка-тригер у bounce homework |
| `GPIO19/GPIO20` | Native USB D-/D+ |
| `GPIO35-GPIO37` | Не використовувати без перевірки; у джерелах позначені як internal flash/PSRAM-related |
| `GPIO43/GPIO44` | UART0 TX/RX у ESP32-S3; можуть бути зайняті serial |
| `GPIO48` | Очікуваний WS2812-style onboard RGB LED |

## Power rules

- GPIO тільки `3.3V`.
- Button modules з `VCC/OUT/GND` живити від `3V3`, щоб `OUT` не став `5V`.
- LED завжди підключати через резистор `220-330 Ohm`.
- Перед зовнішнім живленням через header перевірити мультиметром `5V`, `3V3` і
  `GND` саме на цій платі.
- У community notes є попередження, що на деяких платах pin `5V` поводився як
  ще один `3V3`; перевіряти, не припускати.

## Jumpers and onboard devices

- Є onboard RGB LED; очікувано WS2812-compatible на `GPIO48`.
- На community-схемах/описах фігурують jumpers `RGB`, `IN-OUT`, `USB-OTG`.
- Не паяти jumpers без конкретної задачі:
  - `RGB` може впливати на підключення onboard LED.
  - `IN-OUT` пов'язаний з VBUS/5Vin шляхом живлення.
  - `USB-OTG` пов'язаний з USB VBUS між Type-C портами.

## Local references

- `docs/reference/schematics/YD-ESP32-S3-SCH-V1.4.pdf`
- `docs/reference/images/yd-esp32-s3-devkitc-1-clone-pinout.jpg`
- `docs/reference/datasheets/esp32-s3_datasheet_en.pdf`
- `docs/reference/datasheets/esp32-s3-wroom-1_wroom-1u_datasheet_en.pdf`
