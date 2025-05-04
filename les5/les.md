# Лекция 5. Условные конструкции: if-else, match.

## 1. Оператор if-else

Базовый условный оператор в Rust:

```rust
fn main() {
    let number = 10;
    
    if number > 0 {
        println!("Число положительное");
    } else if number < 0 {
        println!("Число отрицательное");
    } else {
        println!("Число равно нулю");
    }
}
```

### Особенности:
- Условие должно быть строго типа `bool` (не преобразуется автоматически как в C/C++)
- Можно использовать как выражение (возвращает значение)

```rust
fn main() {
    let condition = true;
    let x = if condition { 5 } else { 6 };
    
    println!("Значение x: {}", x); // 5
}
```

## 2. Множественная проверка (else if)

Для проверки нескольких условий:

```rust
fn main() {
    let score = 85;
    
    if score >= 90 {
        println!("Отлично!");
    } else if score >= 75 {
        println!("Хорошо!");
    } else if score >= 60 {
        println!("Удовлетворительно");
    } else {
        println!("Неудовлетворительно");
    }
}
```

## 3. Тернарный оператор (аналог)

Rust не имеет классического тернарного оператора, но использует if как выражение:

```rust
fn main() {
    let age = 20;
    let status = if age >= 18 { "взрослый" } else { "ребёнок" };
    
    println!("Статус: {}", status);
}
```

## 4. Оператор match

Мощная альтернатива цепочкам if-else:

```rust
fn main() {
    let number = 3;
    
    match number {
        1 => println!("Один"),
        2 | 3 | 5 | 7 => println!("Простое число"),
        4..=10 => println!("От 4 до 10"),
        _ => println!("Другое число"),
    }
}
```

### Использование с возвратом значения:

```rust
fn main() {
    let boolean = true;
    
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    
    println!("{} -> {}", boolean, binary);
}
```

### Деструктуризация в match:

```rust
fn main() {
    let pair = (0, -2);
    
    match pair {
        (0, y) => println!("Первое 0, y = {}", y),
        (x, 0) => println!("x = {}, второе 0", x),
        _ => println!("Оба ненулевые"),
    }
}
```

## 5. Итог

1. **if-else**: базовый условный оператор, может возвращать значение
2. **else if**: множественные проверки условий
3. **Тернарный оператор**: заменяется на if-else выражение
4. **match**: мощное сопоставление с образцом, поддерживает диапазоны, деструктуризацию

🚀 **Ресурсы:**
- Изучение Rust с Гоша Дударь [Видео](https://www.youtube.com/watch?v=JW146pBzFRo&list=PL0lO_mIqDDFU_3UaxCF6p98ELxXpAyHpW&index=5)
- Книга "The Rust Programming Language" (глава 3, 6)