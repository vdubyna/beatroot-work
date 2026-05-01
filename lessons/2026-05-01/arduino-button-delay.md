# Arduino / ESP32: кнопка, interrupt та debounce

Цей код схожий на скетч для Arduino framework у PlatformIO. Він написаний мовою C++ і призначений для ESP32 / ESP32-S3.

Нова версія коду вже не перевіряє кнопку вручну в `loop()`. Замість цього вона використовує interrupt: спеціальна функція автоматично викликається в момент, коли сигнал на піні кнопки змінюється.

## Код зі скріншота

На скріншоті видно приблизно таку логіку:

```cpp
#define BUTTON_RIGHT 1

// int16_t counter_left = 0;
int16_t counter_right = 0;

uint32_t debounce = 0;

void IRAM_ATTR addition_reaction_right() {
    if (millis() - debounce > 100) {
        debounce = millis() - 80;
    }
}

void IRAM_ATTR reaction_right() {
    if (millis() - debounce > 100) {
        debounce = millis();
        counter_right++;
    }
}

void setup() {
    // pinMode(BUTTON_LEFT, INPUT);
    pinMode(BUTTON_RIGHT, INPUT);
    debounce = millis();
    Serial.begin(115200);

    attachInterrupt(
        digitalPinToInterrupt(BUTTON_RIGHT),
        addition_reaction_right,
        RISING
    );

    attachInterrupt(
        digitalPinToInterrupt(BUTTON_RIGHT),
        reaction_right,
        FALLING
    );
}

void loop() {
    // Основна програма працює тут.
}
```

## На який пін підключати кнопку

У коді є рядок:

```cpp
#define BUTTON_RIGHT 1
```

Це означає, що кнопка `BUTTON_RIGHT` підключена до `GPIO1`.

На нашій платі треба шукати фізичний пін із підписом `1` або `GPIO1`. Саме до нього підключається сигнальна ніжка кнопки.

Рекомендований простий варіант підключення:

| Ніжка кнопки | Куди підключити |
| --- | --- |
| Перша ніжка | `GPIO1`, пін із підписом `1` |
| Друга ніжка | `GND` |

Але тоді в коді краще використовувати не `INPUT`, а `INPUT_PULLUP`:

```cpp
pinMode(BUTTON_RIGHT, INPUT_PULLUP);
```

При такому підключенні:

- коли кнопка не натиснута, GPIO1 має стан `HIGH`;
- коли кнопка натиснута, GPIO1 замикається на `GND` і має стан `LOW`;
- натискання кнопки створює перехід `HIGH -> LOW`, тобто `FALLING`;
- відпускання кнопки створює перехід `LOW -> HIGH`, тобто `RISING`.

Важливо: не підключай GPIO напряму до `5V`. GPIO ESP32 працюють із логікою `3.3V`.

## Чому `INPUT_PULLUP` краще за `INPUT`

У скріншоті зараз написано:

```cpp
pinMode(BUTTON_RIGHT, INPUT);
```

Для кнопки це небезпечно з точки зору логіки: коли кнопка не натиснута, пін може бути ні до чого не підключений. Такий стан називається floating input. Контролер може випадково читати то `HIGH`, то `LOW`, навіть якщо кнопку ніхто не чіпає.

Щоб сигнал був стабільний, потрібне підтягування:

- або внутрішнє підтягування через `INPUT_PULLUP`;
- або зовнішній резистор, зазвичай близько `10k`.

Для навчального підключення найпростіше:

```cpp
pinMode(BUTTON_RIGHT, INPUT_PULLUP);
```

і кнопка між `GPIO1` та `GND`.

## Що таке interrupt

Ось ці рядки підключають interrupt:

```cpp
attachInterrupt(digitalPinToInterrupt(BUTTON_RIGHT), addition_reaction_right, RISING);
attachInterrupt(digitalPinToInterrupt(BUTTON_RIGHT), reaction_right, FALLING);
```

Interrupt - це реакція мікроконтролера на подію. У цьому випадку подія - зміна сигналу на піні кнопки.

Замість того щоб постійно питати в `loop()`:

```cpp
digitalRead(BUTTON_RIGHT)
```

мікроконтролер сам викликає потрібну функцію, коли на GPIO1 стається фронт сигналу.

## Що таке `RISING` і `FALLING`

`RISING` означає перехід сигналу з `LOW` у `HIGH`.

`FALLING` означає перехід сигналу з `HIGH` у `LOW`.

Якщо кнопка підключена між `GPIO1` і `GND`, а пін налаштований як `INPUT_PULLUP`, то:

| Дія | Стан сигналу | Interrupt |
| --- | --- | --- |
| Натиснули кнопку | `HIGH -> LOW` | `FALLING` |
| Відпустили кнопку | `LOW -> HIGH` | `RISING` |

У такій схемі `reaction_right()` викликається саме на натискання.

## Для чого `debounce`

Змінна:

```cpp
uint32_t debounce = 0;
```

зберігає час останньої прийнятої події від кнопки.

Кнопка механічна. Коли її натискають або відпускають, контакт не завжди перемикається ідеально один раз. Він може кілька мілісекунд "дрижати": дуже швидко давати `HIGH`, `LOW`, `HIGH`, `LOW`.

Це називають:

- button bounce;
- contact bounce;
- jitter;
- українською: дрижання контактів.

Оцей код відсікає занадто часті події:

```cpp
if (millis() - debounce > 100) {
    debounce = millis();
    counter_right++;
}
```

Логіка така: якщо з минулої події минуло більше `100` мс, натискання вважається справжнім. Якщо минуло менше, це скоріше за все дрижання контакту, і його ігноруємо.

## Що робить `reaction_right()`

```cpp
void IRAM_ATTR reaction_right() {
    if (millis() - debounce > 100) {
        debounce = millis();
        counter_right++;
    }
}
```

Ця функція збільшує лічильник правої кнопки:

```cpp
counter_right++;
```

Але тільки якщо пройшло більше `100` мс після попередньої події. Тобто вона рахує не кожен електричний імпульс, а тільки стабільні натискання.

## Що робить `addition_reaction_right()`

```cpp
void IRAM_ATTR addition_reaction_right() {
    if (millis() - debounce > 100) {
        debounce = millis() - 80;
    }
}
```

Ця функція викликається на протилежний фронт сигналу. Якщо використовуємо `INPUT_PULLUP`, то це буде відпускання кнопки.

Рядок:

```cpp
debounce = millis() - 80;
```

виглядає як ручне підлаштування debounce-вікна. Тобто код не просто ставить поточний час, а спеціально зсуває його на `80` мс назад.

Практичний ефект: після відпускання кнопки наступне натискання може бути прийняте швидше, ніж через повні `100` мс.

Це вже тонке налаштування, але для першої версії воно не обов'язкове. Для навчання простіше мати одну функцію, яка реагує тільки на натискання.

## Важливе зауваження про два `attachInterrupt`

У коді двічі викликається `attachInterrupt()` для одного й того самого піна `BUTTON_RIGHT`:

```cpp
attachInterrupt(digitalPinToInterrupt(BUTTON_RIGHT), addition_reaction_right, RISING);
attachInterrupt(digitalPinToInterrupt(BUTTON_RIGHT), reaction_right, FALLING);
```

Це може бути проблемою. У багатьох Arduino/ESP32 реалізаціях для одного GPIO очікується один handler interrupt. Другий виклик може перезаписати перший або поводитися не так, як ми очікуємо.

Якщо треба рахувати тільки натискання, достатньо одного interrupt:

```cpp
pinMode(BUTTON_RIGHT, INPUT_PULLUP);
attachInterrupt(
    digitalPinToInterrupt(BUTTON_RIGHT),
    reaction_right,
    FALLING
);
```

Якщо треба бачити і натискання, і відпускання, краще використовувати `CHANGE`:

```cpp
attachInterrupt(
    digitalPinToInterrupt(BUTTON_RIGHT),
    reaction_right_change,
    CHANGE
);
```

а всередині функції вже читати поточний стан піна через `digitalRead(BUTTON_RIGHT)`.

## Що таке `IRAM_ATTR`

```cpp
void IRAM_ATTR reaction_right()
```

`IRAM_ATTR` каже ESP32 покласти цю функцію в швидку внутрішню RAM. Для interrupt-функцій на ESP32 це нормальна практика, бо interrupt може спрацювати в момент, коли доступ до flash-пам'яті небажаний або тимчасово недоступний.

Простими словами: це спеціальна позначка для функцій, які мають швидко і надійно виконуватись під час interrupt.

## Короткий висновок

Цей код:

- використовує GPIO1 як пін правої кнопки;
- реагує на зміну сигналу через interrupt;
- рахує натискання в `counter_right`;
- використовує `debounce`, щоб відсікти дрижання контактів;
- має бути підключений до кнопки через стабільну схему з підтягуванням.

Для нашої плати найпростіше підключення: кнопка між піном `1` / `GPIO1` і `GND`, а в коді змінити `INPUT` на `INPUT_PULLUP`.
