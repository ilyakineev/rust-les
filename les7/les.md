# Лекция 7. Функции и модули в Rust

## 1. Создание функций

### Базовый синтаксис
```rust
// Объявление функции
fn greet() {
    println!("Привет, мир!");
}

// Функция с параметрами
fn print_sum(a: i32, b: i32) {
    println!("Сумма: {}", a + b);
}

fn main() {
    greet();
    print_sum(5, 3);
}
```

### Особенности:
- Ключевое слово `fn` для объявления
- Типы параметров указываются явно
- Змеиный_регистр (snake_case) для имён функций

## 2. Передача по ссылке

### Неизменяемые ссылки (&)
```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("Длина '{}': {}", s, len);
}
```

### Изменяемые ссылки (&mut)
```rust
fn append_world(s: &mut String) {
    s.push_str(", world!");
}

fn main() {
    let mut s = String::from("hello");
    append_world(&mut s);
    println!("{}", s); // "hello, world!"
}
```

## 3. Возвращение значений

### Простое возвращение
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b // без точки с запятой = возврат
}

fn main() {
    let sum = add(2, 3);
    println!("2 + 3 = {}", sum);
}
```

### Возврат нескольких значений через кортеж
```rust
fn calculate(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}

fn main() {
    let (sum, product) = calculate(4, 5);
    println!("Сумма: {}, Произведение: {}", sum, product);
}
```

## 4. Передача кортежа

### Приём кортежа как аргумента
```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Координаты: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

### Возврат кортежа из функции
```rust
fn min_max(values: &[i32]) -> (i32, i32) {
    let mut min = values[0];
    let mut max = values[0];
    
    for &value in values {
        if value < min { min = value; }
        if value > max { max = value; }
    }
    
    (min, max)
}

fn main() {
    let numbers = [4, 2, 9, 1, 5];
    let (min, max) = min_max(&numbers);
    println!("Min: {}, Max: {}", min, max);
}
```

## 5. Работа с макросами

### Определение простого макроса
```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("Вызвана функция {:?}()", stringify!($func_name));
        }
    };
}

fn main() {
    say_hello!();
    
    create_function!(foo);
    foo();
}
```

### Макрос с параметрами
```rust
macro_rules! vec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let v = vec![1, 2, 3];
    println!("{:?}", v);
}
```

## 6. Создание модуля

### Структура проекта:
```
my_project/
├── Cargo.toml
└── src/
    ├── main.rs
    └── math/
        ├── mod.rs
        └── operations.rs
```

### math/mod.rs:
```rust
pub mod operations; // Подключение подмодуля

pub fn greet() {
    println!("Добро пожаловать в математический модуль!");
}
```

### math/operations.rs:
```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
```

### Использование в main.rs:
```rust
mod math; // Подключение модуля

use math::operations::{add, multiply};

fn main() {
    math::greet();
    println!("2 + 3 = {}", add(2, 3));
    println!("2 * 3 = {}", multiply(2, 3));
}
```

## 7. Итог

1. **Функции**: 
   - Объявляются через `fn`
   - Поддерживают передачу по значению и по ссылке
   - Могут возвращать значения, включая кортежи

2. **Ссылки**:
   - `&` - неизменяемая ссылка
   - `&mut` - изменяемая ссылка
   - Обеспечивают безопасность памяти

3. **Кортежи**:
   - Удобны для передачи и возврата нескольких значений
   - Поддерживают деструктуризацию

4. **Макросы**:
   - Определяются через `macro_rules!`
   - Обрабатываются на этапе компиляции
   - Позволяют создавать DSL (предметно-ориентированные языки)

5. **Модули**:
   - Организуют код в логические единицы
   - Управляют видимостью через `pub`
   - Поддерживают иерархическую структуру

🚀 **Ресурсы:**
- Изучение Rust с Гоша Дударь [Видео](https://www.youtube.com/watch?v=LJ15QWqnJAk&list=PL0lO_mIqDDFU_3UaxCF6p98ELxXpAyHpW&index=7)