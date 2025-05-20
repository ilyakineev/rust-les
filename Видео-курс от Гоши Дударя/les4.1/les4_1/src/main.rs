use std::io;
use std::fs;
use std::env;

fn main() {
    // ==============================================
    // Демонстрация заимствования переменных
    // ==============================================
    println!("\n\x1b[36m=== Демонстрация заимствования переменных ===\x1b[0m");

    // 1. Неизменяемое заимствование
    println!("\n\x1b[34m1. Неизменяемое заимствование:\x1b[0m");
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("Длина '\x1b[33m{}\x1b[0m' равна \x1b[33m{}\x1b[0m", s, len);

    // 2. Изменяемое заимствование
    println!("\n\x1b[34m2. Изменяемое заимствование:\x1b[0m");
    let mut s = String::from("hello");
    change_string(&mut s);
    println!("Изменённая строка: \x1b[33m{}\x1b[0m", s);

    // 3. Конфликты заимствования
    println!("\n\x1b[34m3. Конфликты заимствования:\x1b[0m");
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("r1: \x1b[33m{}\x1b[0m, r2: \x1b[33m{}\x1b[0m", r1, r2);
    let r3 = &mut s;
    r3.push_str(", world!");
    println!("r3: \x1b[33m{}\x1b[0m", r3);

    // 4. Заимствование в циклах
    println!("\n\x1b[34m4. Заимствование в циклах:\x1b[0m");
    let mut vec = vec![1, 2, 3];
    println!("Исходный вектор: \x1b[33m{:?}\x1b[0m", vec);
    
    println!("Чтение:");
    for i in &vec {
        println!("  Элемент: \x1b[33m{}\x1b[0m", i);
    }
    
    println!("Изменение:");
    for i in &mut vec {
        *i += 10;
        println!("  Изменённый элемент: \x1b[33m{}\x1b[0m", i);
    }
    println!("Итоговый вектор: \x1b[33m{:?}\x1b[0m", vec);

    // 5. Заимствование срезов
    println!("\n\x1b[34m5. Заимствование срезов:\x1b[0m");
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("Срезы: '\x1b[33m{}\x1b[0m' и '\x1b[33m{}\x1b[0m'", hello, world);

    // 6. Возврат заимствований из функций
    println!("\n\x1b[34m6. Возврат заимствований из функций:\x1b[0m");
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("Первое слово: '\x1b[33m{}\x1b[0m'", word);

    // ==============================================
    // Демонстрационный пример
    // ==============================================
    println!("\n\x1b[36m=== Итоговый демонстрационный пример ===\x1b[0m");
    let mut data = vec![10, 20, 30];
    
    println!("Исходные данные: \x1b[33m{:?}\x1b[0m", data);
    print_data(&data);
    modify_data(&mut data);
    println!("Модифицированные данные: \x1b[33m{:?}\x1b[0m", data);
}

// Вспомогательные функции
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_string(s: &mut String) {
    s.push_str(", world!");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn print_data(data: &Vec<i32>) {
    println!("Чтение данных:");
    for val in data {
        println!("  Значение: \x1b[33m{}\x1b[0m", val);
    }
}

fn modify_data(data: &mut Vec<i32>) {
    println!("Модификация данных...");
    for val in data.iter_mut() {
        *val *= 2;
    }
}