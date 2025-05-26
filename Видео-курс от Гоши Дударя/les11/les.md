# Лекция X. Структуры, Перечисления и Обобщения в Rust

## 1. Типы структур

### Обычные структуры (`struct`)

```rust
struct Person {
    name: String,
    age: u32,
}
```

### Кортежные структуры

```rust
struct Color(u8, u8, u8);
```

### Единичные (unit-like) структуры

```rust
struct Marker;
```

### Особенности:

* Используются для группировки связанных данных.
* Позволяют реализовывать методы и трейты.
* Объявляются через `struct`.

---

## 2. Пример использования структур

```rust
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }

    fn greet(&self) {
        println!("Привет, меня зовут {} и мне {} лет", self.name, self.age);
    }
}

fn main() {
    let user = Person::new("Алиса", 30);
    user.greet();
}
```

---

## 3. Перечисления (`enum`)

### Пример с `enum`

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_player(direction: Direction) {
    match direction {
        Direction::Up => println!("Движение вверх"),
        Direction::Down => println!("Движение вниз"),
        Direction::Left => println!("Движение влево"),
        Direction::Right => println!("Движение вправо"),
    }
}

fn main() {
    move_player(Direction::Left);
}
```

### Перечисление с данными

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };

    match msg {
        Message::Quit => println!("Выход"),
        Message::Move { x, y } => println!("Перемещение: ({}, {})", x, y),
        Message::Write(text) => println!("Сообщение: {}", text),
    }
}
```

---

## 4. Обобщения (Generics)

### Функция с обобщением

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let numbers = vec![1, 2, 3, 9, 5];
    println!("Наибольшее число: {}", largest(&numbers));
}
```

### Обобщённая структура

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let int_point = Point { x: 1, y: 2 };
    let float_point = Point { x: 1.0, y: 2.0 };
}
```

---

## 5. Итог

1. **Структуры (`struct`)**:

   * Используются для объединения данных.
   * Поддерживают методы и инкапсуляцию.
   * Типы: обычные, кортежные, unit-like.

2. **Перечисления (`enum`)**:

   * Позволяют описывать набор возможных состояний.
   * Могут содержать данные разных типов.
   * Используются вместе с `match`.

3. **Обобщения (`generics`)**:

   * Позволяют писать универсальный код.
   * Используются в функциях, структурах и перечислениях.
   * Обеспечивают безопасность типов без дублирования кода.

🚀 **Ресурсы:**

* Rust Book — [Глава «Structs»](https://doc.rust-lang.ru/book/ch05-00-structs.html)
* Rust Book — [Глава «Enums»](https://doc.rust-lang.ru/book/ch06-00-enums.html)
* Rust Book — [Глава «Generics»](https://doc.rust-lang.ru/book/ch10-00-generics.html)
* YouTube: Гоша Дударь — [Изучение Rust (видео)](https://www.youtube.com/watch?v=Z8IhYLX6P04&list=PL0lO_mIqDDFU_3UaxCF6p98ELxXpAyHpW&index=11)