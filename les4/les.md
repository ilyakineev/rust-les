# Лекция 4. Управление памятью и владение (Ownership).

## 1. Получение данных

В Rust есть несколько способов получения данных:

### Чтение из консоли
```rust
use std::io;

fn main() {
    println!("Введите ваше имя:");
    
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Не удалось прочитать строку");
    
    println!("Привет, {}!", name.trim());
}
```

### Чтение из файла
```rust
use std::fs;

fn main() {
    let content = fs::read_to_string("example.txt")
        .expect("Не удалось прочитать файл");
    
    println!("Содержимое файла: {}", content);
}
```

### Получение аргументов командной строки
```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        println!("Первый аргумент: {}", args[1]);
    }
}
```

## 2. Взаимодействие с пользователем

### Базовый ввод-вывод
```rust
fn main() {
    println!("Это обычный вывод");
    eprintln!("Это вывод в stderr (для ошибок)");
    
    print!("Этот текст не переведёт строку");
    println!(" и этот текст будет на той же строке");
}
```

### Форматированный вывод
```rust
fn main() {
    let name = "Анна";
    let age = 30;
    
    println!("Имя: {}, Возраст: {}", name, age);
    println!("Имя: {0}, Возраст: {1}, снова имя: {0}", name, age);
    println!("Имя: {name}, Возраст: {age}", name=name, age=age);
}
```

## 3. Преобразование данных

### Преобразование между типами
```rust
fn main() {
    // Из строки в число
    let num_str = "42";
    let num: i32 = num_str.parse().expect("Не число!");
    println!("Число: {}", num);
    
    // Из числа в строку
    let num = 42;
    let num_str = num.to_string();
    println!("Строка: {}", num_str);
    
    // Явное приведение типов
    let x = 42u32;
    let y = x as i64;
    println!("Преобразованное число: {}", y);
}
```

## 5. Итог

1. **Получение данных**: Rust предоставляет различные способы получения данных - из консоли, файлов, аргументов командной строки.
2. **Взаимодействие с пользователем**: форматированный ввод-вывод через макросы `println!`, `eprintln!`, `print!`.
3. **Преобразование данных**: преобразование между типами, работа с `Option` и `Result` для обработки возможных ошибок.

🚀 **Ресурсы:**
- Изучение Rust с Гоша Дударь [Видео](https://www.youtube.com/watch?v=lDsd6UaMo-M&list=PL0lO_mIqDDFU_3UaxCF6p98ELxXpAyHpW&index=4)