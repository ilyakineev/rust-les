# Лекция 6. Циклы и операторы.

## 1. Цикл `for`

Основной цикл для итераций в Rust:

### Перебор диапазона
```rust
fn main() {
    // Перебор чисел от 1 до 5 (включительно)
    for i in 1..=5 {
        println!("i = {}", i);
    }
    
    // Перебор чисел от 1 до 4 (не включая 5)
    for i in 1..5 {
        println!("i = {}", i);
    }
}
```

### Перебор массива
```rust
fn main() {
    let numbers = [10, 20, 30, 40, 50];
    
    // Простой перебор элементов
    for num in numbers {
        println!("Число: {}", num);
    }
    
    // Перебор с индексом
    for (index, num) in numbers.iter().enumerate() {
        println!("Индекс: {}, Число: {}", index, num);
    }
}
```

### Перебор строки
```rust
fn main() {
    let text = "Привет";
    
    for c in text.chars() {
        println!("Символ: {}", c);
    }
}
```

## 2. Цикл `while`

Цикл с условием продолжения:

```rust
fn main() {
    let mut counter = 5;
    
    while counter > 0 {
        println!("{}!", counter);
        counter -= 1;
    }
    
    println!("Старт!");
}
```

### Пример с условием
```rust
fn main() {
    let mut number = 3;
    
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
}
```

## 3. Операторы в циклах

### `break` - прерывание цикла
```rust
fn main() {
    let mut sum = 0;
    
    for i in 1..100 {
        sum += i;
        if sum > 100 {
            println!("Достигнут предел на i={}", i);
            break;
        }
    }
}
```

### `continue` - переход к следующей итерации
```rust
fn main() {
    for i in 1..10 {
        if i % 2 == 0 {
            continue; // пропускаем чётные числа
        }
        println!("Нечётное: {}", i);
    }
}
```

## 4. Цикл `loop`

Бесконечный цикл с явным выходом через `break`:

```rust
fn main() {
    let mut count = 0;
    
    loop {
        count += 1;
        println!("Счёт: {}", count);
        
        if count == 5 {
            break;
        }
    }
}
```

### Возврат значения из loop
```rust
fn main() {
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("Результат: {}", result); // 20
}
```

## 5. Итог

1. **for**: основной цикл для итераций по диапазонам, массивам, коллекциям
2. **while**: цикл с условием продолжения
3. **Операторы**: 
   - `break` - выход из цикла
   - `continue` - переход к следующей итерации
4. **loop**: бесконечный цикл с возможностью возврата значения

🚀 **Ресурсы:**
- Изучение Rust с Гоша Дударь [Видео](https://www.youtube.com/watch?v=py2E_OEtujA&list=PL0lO_mIqDDFU_3UaxCF6p98ELxXpAyHpW&index=6)