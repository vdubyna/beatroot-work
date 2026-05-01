# Домашня робота: поліцейська мигалка на ESP32

Проєкт на Rust для ESP32-S3. Програма керує двома зовнішніми світлодіодами:
червоний підключений до `GPIO4`, синій підключений до `GPIO5`.

## Деталі

| Деталь | Кількість | Примітка |
| --- | ---: | --- |
| ESP32-S3 dev board | 1 | наша плата `YD-ESP32-23` |
| Breadboard | 1 | зручно для макету |
| Червоний LED | 1 | звичайний 3/5 мм |
| Синій LED | 1 | звичайний 3/5 мм |
| Резистор 220-330 Ом | 2 | по одному на кожен LED |
| Jumper wires | 4-6 | male-male або під вашу breadboard |
| USB-C кабель | 1 | для живлення і прошивки через порт `COM` |

Резистори обов'язкові: не підключай LED напряму до GPIO.

## Підключення

| ESP32 | Через що | LED |
| --- | --- | --- |
| `GPIO4` / пін `4` | резистор 220-330 Ом | довга ніжка червоного LED |
| коротка ніжка червоного LED | дріт | `GND` |
| `GPIO5` / пін `5` | резистор 220-330 Ом | довга ніжка синього LED |
| коротка ніжка синього LED | дріт | `GND` |

На breadboard це виглядає так:

1. Встав червоний LED так, щоб його ніжки були в різних рядах.
2. Довгу ніжку червоного LED з'єднай з одним кінцем резистора.
3. Другий кінець резистора з'єднай дротом з піном `4` / `GPIO4` на ESP32.
4. Коротку ніжку червоного LED з'єднай з `GND`.
5. Так само для синього LED: довга ніжка через резистор до `5` / `GPIO5`, коротка ніжка до `GND`.
6. Якщо використовуєш мінусову рейку breadboard, спочатку з'єднай її з `GND` ESP32, а короткі ніжки LED вставляй у цю рейку.

Якщо LED не світиться, перевір полярність: довга ніжка зазвичай `+` / анод,
коротка ніжка і пласка сторона корпусу зазвичай `-` / катод.

## Як працює програма

Патерн трохи ускладнений: червоний LED швидко блимає 3 рази, потім синій LED
швидко блимає 3 рази. Після цього цикл повторюється без зупинки.

Швидкість можна змінити в `src/bin/main.rs`:

- `FLASH_MS` - скільки LED світиться;
- `GAP_MS` - пауза між короткими спалахами;
- `SWITCH_GAP_MS` - пауза між червоною і синьою серіями;
- `BURSTS_PER_COLOR` - кількість спалахів одного кольору.

## Чим це відрізняється від Android і C++

У цьому проєкті Rust-код працює прямо на ESP32. Тут немає Android/Linux/Windows
між програмою і залізом: firmware напряму налаштовує GPIO-піни і виставляє на
них `HIGH` або `LOW`.

На Android звичайний додаток не може просто сказати телефону: "дай 3.3V на
GPIO4". У більшості Android-пристроїв немає доступних GPIO для додатків. Тому
Android-версія зазвичай була б або лише візуальною симуляцією на екрані, або
керувала б зовнішнім мікроконтролером через USB, Bluetooth/Wi-Fi, BLE чи інший
інтерфейс.

На C++ для ESP32 найчастіше використовують Arduino-style API: `pinMode`,
`digitalWrite`, `delay`. Ідея та сама, але Rust сильніше контролює типи й
володіння ресурсами: GPIO-пін у коді є конкретним об'єктом, який не можна
випадково використати у двох місцях одночасно.

Коротко:

| Варіант | Де працює код | Як керує LED |
| --- | --- | --- |
| Rust + ESP32 | прямо на мікроконтролері | напряму через GPIO |
| Android app | на телефоні/планшеті | зазвичай тільки симуляція або команда зовнішній платі |
| C++ Arduino + ESP32 | прямо на мікроконтролері | напряму через GPIO |

## Приклад для Android, Kotlin

Це не керує фізичними GPIO телефона. Це тільки візуальна симуляція мигалки в
Android-додатку: два кружечки на екрані по черзі стають яскравими.

```kotlin
class MainActivity : AppCompatActivity() {
    private lateinit var redLight: View
    private lateinit var blueLight: View

    private val handler = Handler(Looper.getMainLooper())
    private var step = 0

    private val flasher = object : Runnable {
        override fun run() {
            when (step) {
                0, 2, 4 -> show(redOn = true, blueOn = false)
                1, 3, 5 -> show(redOn = false, blueOn = false)
                6, 8, 10 -> show(redOn = false, blueOn = true)
                else -> show(redOn = false, blueOn = false)
            }

            step = (step + 1) % 12
            handler.postDelayed(this, if (step == 6 || step == 0) 180 else 90)
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        redLight = findViewById(R.id.redLight)
        blueLight = findViewById(R.id.blueLight)

        handler.post(flasher)
    }

    override fun onDestroy() {
        handler.removeCallbacks(flasher)
        super.onDestroy()
    }

    private fun show(redOn: Boolean, blueOn: Boolean) {
        redLight.alpha = if (redOn) 1.0f else 0.2f
        blueLight.alpha = if (blueOn) 1.0f else 0.2f
    }
}
```

Якби Android мав керувати ESP32 реально, Android-код зазвичай відправляв би
команди типу `"RED_ON"` / `"BLUE_ON"` через USB/Bluetooth/Wi-Fi, а ESP32 вже
перемикав би GPIO.

## Приклад для C++, Arduino-style ESP32

Це вже ближче до нашого Rust-проєкту, бо код теж працює прямо на ESP32 і
напряму керує GPIO.

```cpp
const int RED_PIN = 4;
const int BLUE_PIN = 5;

const int FLASH_MS = 90;
const int GAP_MS = 70;
const int SWITCH_GAP_MS = 180;
const int BURSTS_PER_COLOR = 3;

void flashColor(int activePin, int inactivePin) {
  digitalWrite(inactivePin, LOW);

  for (int i = 0; i < BURSTS_PER_COLOR; i++) {
    digitalWrite(activePin, HIGH);
    delay(FLASH_MS);

    digitalWrite(activePin, LOW);
    delay(GAP_MS);
  }
}

void setup() {
  pinMode(RED_PIN, OUTPUT);
  pinMode(BLUE_PIN, OUTPUT);

  digitalWrite(RED_PIN, LOW);
  digitalWrite(BLUE_PIN, LOW);
}

void loop() {
  flashColor(RED_PIN, BLUE_PIN);
  delay(SWITCH_GAP_MS);

  flashColor(BLUE_PIN, RED_PIN);
  delay(SWITCH_GAP_MS);
}
```

## Запуск

З теки цієї домашки:

```sh
cargo build
```

Прошити плату і відкрити serial monitor:

```sh
cargo run --bin police-flasher
```

Конфігурація вже налаштована на порт `/dev/cu.usbmodem2101`.
Якщо macOS покаже інший порт, зміни його в `.cargo/config.toml`.
