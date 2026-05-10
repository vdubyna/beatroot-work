# LLM Context Snapshot

Оновлено: 2026-05-10.

## Репозиторій

- Навчальний embedded-репозиторій для ESP32-S3.
- Основна мова для домашніх робіт: Rust.
- Основний стек: `cargo`, `esp-hal`, `esp-bootloader-esp-idf`, `esp-println`,
  `espflash`.
- Не створювати Arduino, PlatformIO або `.ino` проєкти без прямого прохання.

## Основна плата

- Плата: `ESP32-S3 YD-ESP32-23`, на фото маркування `2022-V1.3`.
- Модуль: маркування `S3-N16R8`; очікувано `ESP32-S3-WROOM-1-N16R8`.
- Confirmed on board через `espflash`: chip `esp32s3`, revision `v0.2`,
  crystal `40 MHz`, flash `16MB`, features `WiFi`, `BLE`, `Embedded Flash`.
- PSRAM `8MB` очікується з `N16R8`, але ще треба перевірити прошивкою.
- Основний порт прошивки в актуальних проєктах: `/dev/cu.usbserial-A5069RR4`.

## GPIO quick facts

- GPIO логіка: `3.3V`; не подавати `5V` на GPIO.
- ADC1 на ESP32-S3: `GPIO1-GPIO10`.
- Для аналогових домашок краще починати з ADC1, не з ADC2.
- Для напруги через `esp-hal` використовувати `enable_pin_with_cal` з
  `AdcCalCurve`; інакше заземлений ADC-пін може показувати великий offset
  приблизно `1.6-1.8V`.
- Не переносити механічно поради для класичного ESP32, де часто радять
  `GPIO32-GPIO39` для ADC.
- Вбудований WS2812-style RGB LED очікувано на `GPIO48`.
- `GPIO0` пов'язаний з boot mode.
- `GPIO19/GPIO20` пов'язані з native USB / USB OTG.
- `GPIO35-GPIO37` у джерелах для цієї плати позначені як небезпечні/зарезервовані
  для внутрішньої flash/PSRAM комунікації; не використовувати без окремої
  перевірки.

## Доступні інструменти

- Мультиметр.
- Логічний аналізатор USB; для macOS практично обрано Saleae Logic 2.
- Осцилограф `PicoScope 2206B MSO`.

## Поточні проєкти

- `examples/one-led` - RGB LED на `GPIO48`, кнопка на `GPIO1`.
- `homeworks/2026-04-27` - police flasher, LED на `GPIO4` і `GPIO5`.
- `homeworks/2026-04-29` - LED режими, кнопки-модулі на `GPIO6` і `GPIO7`.
- `homeworks/2026-05-01` - штучний bounce: `GPIO17` trigger, `GPIO16`
  transistor drive, `GPIO15` measured input.
- `homeworks/2026-05-04` - LDR/ADC, основний вхід `GPIO4 / ADC1_CH3`,
  індикатор освітлення LED на `GPIO15-GPIO17`.

## Перші локальні файли для читання

- `docs/wiki/hardware/yd-esp32-23.md`
- `docs/wiki/projects/index.md`
- `docs/wiki/workflows/rust-esp32.md`
- `docs/reference/README.md`
