## Конспект лекции: Типы данных, переменные, функции

### 1. Типы данных в Rust

Rust — строго типизированный язык. Тип каждой переменной известен на этапе компиляции. Это позволяет находить ошибки ещё до запуска программы.

#### 1.1. Целые числа

* Знаковые: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
* Беззнаковые: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`

Пример:

```rust
let a: i32 = -42;
let b: u8 = 255;
```

[Официальная документация по числам](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types)

#### 1.2. Числа с плавающей точкой

* `f32` — 32-бит
* `f64` — 64-бит (используется по умолчанию)

```rust
let x = 3.14;
let y: f32 = 2.5;
```

#### 1.3. Логический тип

```rust
let is_valid: bool = true;
```

#### 1.4. Символьный тип

```rust
let letter: char = 'z';
let emoji: char = '😊';
```

#### 1.5. Строки

* `&str`: неизменяемый срез строки
* `String`: выделяемая в куче изменяемая строка

```rust
let s: &str = "Hello";
let mut string = String::from("Hello");
string.push_str(", world!");
```

[Официальная документация по строкам](https://doc.rust-lang.org/book/ch08-02-strings.html)

---

### 2. Переменные

#### 2.1. Объявление

```rust
let x = 5;       // неизменяемая
let mut y = 10;  // изменяемая
y += 5;
```

[Подробнее о переменных](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

#### 2.2. Явное указание типа

```rust
let count: i64 = 100;
```

#### 2.3. Shadowing (переобъявление переменной)

```rust
let x = 5;
let x = x + 1;
let x = "six";
```

---

### 3. Арифметические и логические операции

#### Арифметика

```rust
let a = 10;
let b = 3;
let result = a % b;
```

#### Приведение типов

```rust
let a: i32 = 10;
let b: f64 = 3.0;
let result = a as f64 / b;
```

#### Логика

```rust
let result = true && false || !false;
```

---

### 4. Функции

#### 4.1. Объявление

```rust
fn square(n: i32) -> i32 {
    n * n
}
```

#### 4.2. Особенности

* Параметры и возвращаемые типы должны быть явно указаны
* `return` не обязателен, если последнее выражение возвращается неявно

```rust
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
```

#### 4.3. Без возвращаемого значения

```rust
fn greet() {
    println!("Hello!");
}
```

[Официальная документация по функциям](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

---

### 5. Константы и статические переменные

```rust
const PI: f64 = 3.1415;
static mut COUNTER: u32 = 0;
```

* `const` — компилируемое значение, доступное везде
* `static` — глобальная переменная (изменение возможно только через `unsafe`)

---

### Заключение

* Rust требует точности в работе с типами и переменными, что повышает надёжность
* Функции — основной строительный блок для структурирования кода
* Понимание базовых типов необходимо для любой Rust-программы

---

### Рекомендованные источники

1. [**The Rust Programming Language ("The Book")**](https://doc.rust-lang.org/book/)

   * Глава 3.1: [Переменные и изменяемость](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
   * Глава 3.2: [Типы данных](https://doc.rust-lang.org/book/ch03-02-data-types.html)
   * Глава 3.3: [Функции](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
   * Глава 8.2: [String vs \&str](https://doc.rust-lang.org/book/ch08-02-strings.html)

2. **Rust by Example**
   [https://doc.rust-lang.org/rust-by-example/](https://doc.rust-lang.org/rust-by-example/)

   * [Variables](https://doc.rust-lang.org/rust-by-example/variable.html)
   * [Functions](https://doc.rust-lang.org/rust-by-example/fn.html)
   * [Primitives](https://doc.rust-lang.org/rust-by-example/primitives.html)