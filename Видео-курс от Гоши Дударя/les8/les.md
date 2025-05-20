# Лекция 8. Векторы, строки и коллекции.

## 1. Работа с векторами (`Vec<T>`)

### Создание и инициализация

```rust
fn main() {
    let v1: Vec<i32> = Vec::new(); // Пустой вектор
    let v2 = vec![1, 2, 3];        // Вектор с начальными значениями
}
```

* `Vec<T>` — динамический массив, хранящий элементы одного типа.
* Используйте `vec![]` для удобной инициализации.

### Добавление и удаление элементов

```rust
fn main() {
    let mut numbers = vec![1, 2, 3];
    numbers.push(4);             // Добавление элемента
    numbers.remove(1);           // Удаление элемента по индексу
    println!("{:?}", numbers);   // Вывод: [1, 3, 4]
}
```

* `push()` добавляет элемент в конец вектора.
* `remove(index)` удаляет элемент по указанному индексу.

### Доступ к элементам

```rust
fn main() {
    let colors = vec!["red", "green", "blue"];
    println!("Первый цвет: {}", colors[0]); // Прямой доступ
    match colors.get(3) {
        Some(color) => println!("Цвет: {}", color),
        None => println!("Цвет не найден"),
    }
}
```

* `get(index)` возвращает `Option<&T>`, что позволяет безопасно обрабатывать выход за границы.

### Итерация по вектору

```rust
fn main() {
    let numbers = vec![10, 20, 30];
    for num in &numbers {
        println!("{}", num);
    }
}
```

* Использование ссылки `&numbers` позволяет избежать перемещения владения.

## 2. Работа со строками (`String` и `&str`)

### Создание строк

```rust
fn main() {
    let s1 = String::from("Привет");
    let s2 = "Мир".to_string();
    let s3 = "Rust"; // строковый срез (&str)
}
```

* `String` — изменяемая строка, выделенная в куче.
* `&str` — неизменяемый строковый срез.

### Конкатенация и форматирование

```rust
fn main() {
    let hello = String::from("Привет, ");
    let world = "мир!";
    let greeting = hello + world; // hello перемещается
    println!("{}", greeting);

    let formatted = format!("{} {}", "Hello", "Rust");
    println!("{}", formatted);
}
```

* `+` оператор требует перемещения первого операнда.
* `format!` создает новую строку без перемещения.

### Изменение строки

```rust
fn main() {
    let mut s = String::from("Hello");
    s.push(' ');
    s.push_str("world!");
    println!("{}", s);
}
```

* `push(char)` добавляет символ.
* `push_str(&str)` добавляет строковый срез.

### Срезы строк

```rust
fn main() {
    let s = String::from("Здравствуйте");
    let slice = &s[0..4]; // Внимание: может вызвать панику, если границы не совпадают с границами символов UTF-8
    println!("{}", slice);
}
```

* Срезы должны соответствовать границам символов UTF-8.

## 3. Коллекция `HashMap<K, V>`

### Создание и добавление элементов

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Красные"), 10);
    scores.insert(String::from("Синие"), 50);
}
```

* `HashMap` хранит пары ключ-значение.
* Ключи должны реализовывать трейты `Eq` и `Hash`.

### Доступ к значениям

```rust
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Красные"), 10);

    match scores.get("Красные") {
        Some(score) => println!("Очки: {}", score),
        None => println!("Команда не найдена"),
    }
}
```

* `get(&key)` возвращает `Option<&V>`.

### Обновление значений

```rust
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Красные"), 10);
    scores.insert(String::from("Красные"), 25); // Перезапись значения
}
```

* Повторный вызов `insert` с тем же ключом перезаписывает значение.

### Итерация по `HashMap`

```rust
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Красные"), 10);
    scores.insert(String::from("Синие"), 50);

    for (team, score) in &scores {
        println!("{}: {}", team, score);
    }
}
```

* Итерация по ссылкам на ключи и значения.

## 4. Итог

1. **Векторы (`Vec<T>`)**:

   * Динамические массивы, хранящие элементы одного типа.
   * Поддерживают добавление, удаление и безопасный доступ к элементам.

2. **Строки (`String` и `&str`)**:

   * `String` — изменяемая, выделенная в куче строка.
   * `&str` — неизменяемый строковый срез.
   * Поддерживают конкатенацию, изменение и срезы.

3. **HashMap**:

   * Коллекция пар ключ-значение.
   * Ключи должны реализовывать `Eq` и `Hash`.
   * Поддерживает добавление, доступ, обновление и итерацию по элементам.

🚀 **Ресурсы:**
- Изучение Rust с Гоша Дударь [Видео](https://www.youtube.com/watch?v=kbnWUQqzeRo&list=PL0lO_mIqDDFU_3UaxCF6p98ELxXpAyHpW&index=8)
