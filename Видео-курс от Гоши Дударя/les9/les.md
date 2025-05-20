# Лекция 9. Работа с Option, Result и файлами в Rust

## 1. Перечисление `Option<T>`

`Option<T>` представляет значение, которое может **существовать** (`Some`) или **отсутствовать** (`None`).

```rust
fn main() {
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    match some_number {
        Some(n) => println!("Число: {}", n),
        None => println!("Нет значения"),
    }
}
```

* `Option<T>` полезен для обработки отсутствующих значений без `null`.
* `match`, `if let` и `unwrap_or` позволяют удобно обрабатывать `Option`.

### Примеры

```rust
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}
```

---

## 2. Перечисление `Result<T, E>`

`Result<T, E>` — результат операции, который может быть **успешным** (`Ok(T)`) или **ошибкой** (`Err(E)`).

```rust
fn main() {
    let result: Result<i32, &str> = Ok(10);

    match result {
        Ok(val) => println!("Успех: {}", val),
        Err(e) => println!("Ошибка: {}", e),
    }
}
```

### Полезные методы:

```rust
fn square(n: i32) -> Result<i32, String> {
    if n >= 0 {
        Ok(n * n)
    } else {
        Err("Число должно быть положительным".to_string())
    }
}
```

* `unwrap()` — извлекает `Ok`, вызывает панику при `Err`.
* `unwrap_or(default)` — извлекает значение или возвращает `default`.
* `?` — оператор, который упрощает цепочку `Result`.

---

## 3. Обработка ошибок

### Паника (`panic!`)

```rust
fn main() {
    panic!("Что-то пошло не так!");
}
```

* Прерывает выполнение программы.

### Использование `Result` для управления ошибками

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Деление на ноль".into())
    } else {
        Ok(a / b)
    }
}
```

### Оператор `?`

```rust
fn read_username() -> Result<String, std::io::Error> {
    let mut s = String::new();
    std::fs::File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

* Оператор `?` возвращает ошибку вызывающей функции, если она произошла.

---

## 4. Запись данных в файл

```rust
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = File::create("output.txt")?;
    file.write_all(b"Привет, файл!")?;
    Ok(())
}
```

* `File::create(path)` создает (или перезаписывает) файл.
* `write_all` записывает все данные (в байтах).

---

## 5. Чтение из файла

```rust
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("output.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("Содержимое файла: {}", contents);
    Ok(())
}
```

### Альтернатива: короткий способ

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("output.txt")?;
    println!("Файл: {}", contents);
    Ok(())
}
```

---

## 6. Итог

1. **Перечисление `Option<T>`**:

   * Представляет значение, которое может отсутствовать: `Some(value)` или `None`.
   * Безопасная альтернатива `null`.
   * Обрабатывается через `match`, `if let`, или методы (`unwrap_or`, `map`, `and_then` и т.д.).

2. **Перечисление `Result<T, E>`**:

   * Используется для представления успешного (`Ok`) или ошибочного (`Err`) результата.
   * Позволяет безопасно обрабатывать ошибки без паники.
   * Метод `?` облегчает передачу ошибок вверх по стеку вызовов.

3. **Обработка ошибок**:

   * `panic!` завершает программу — используется в крайних случаях.
   * `Result` и `Option` — предпочтительный способ безопасной обработки ошибок.
   * `unwrap()` и `expect()` — быстрое, но небезопасное извлечение значений.

4. **Чтение из файла**:

   * `File::open` + `read_to_string` или `fs::read_to_string`.
   * Требует обработки `Result`, так как операция может завершиться с ошибкой.

5. **Запись в файл**:

   * `File::create` + `write_all`.
   * Данные записываются в виде байтов (`b"..."`).
   * Важно закрывать файл (происходит автоматически при выходе из области видимости).

🚀 **Ресурсы:**

* YouTube: Гоша Дударь — [Изучение Rust (видео)](https://www.youtube.com/watch?v=0Ta8VEjYSFM&list=PL0lO_mIqDDFU_3UaxCF6p98ELxXpAyHpW&index=9)

