
# Лекция 9. Многопоточность, каналы и async/await в Rust

## 1. Многопоточность (`std::thread`)

### Создание потоков

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Привет из нового потока!");
    });

    println!("Привет из главного потока!");

    handle.join().unwrap(); // Ожидание завершения потока
}
```

### Особенности:

* Поток создаётся с помощью `thread::spawn`.
* Возвращает `JoinHandle`, который можно `.join()`-ить для ожидания завершения потока.

## 2. Передача данных между потоками

### Использование канала

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("Привет из потока");
        tx.send(msg).unwrap(); // Отправка данных
    });

    let received = rx.recv().unwrap(); // Получение
    println!("Получено: {}", received);
}
```

### Особенности:

* `mpsc` означает *multi-producer, single-consumer*.
* `tx` — отправитель (`Sender`), `rx` — получатель (`Receiver`).
* `send()` и `recv()` могут блокировать выполнение или вернуть ошибку.

## 3. Асинхронность: `async` и `await`

### Пример простой async-функции

```rust
async fn say_hello() {
    println!("Привет из async-функции");
}

fn main() {
    let future = say_hello(); // Возвращает Future
    futures::executor::block_on(future); // Блокирующий запуск
}
```

### Асинхронная задержка с `tokio`

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Ждём 1 секунду...");
    sleep(Duration::from_secs(1)).await;
    println!("Прошло 1 секунда!");
}
```

### Асинхронный HTTP-запрос

```rust
use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let resp = reqwest::get("https://www.rust-lang.org").await?;
    println!("Статус: {}", resp.status());
    Ok(())
}
```

### Особенности:

* `async fn` возвращает `Future`, который нужно явно запустить.
* `await` приостанавливает выполнение, пока результат не готов.
* `tokio` или `async-std` — популярные runtime-библиотеки для запуска async-кода.

---

## 4. Итог

1. **Многопоточность (`std::thread`)**:

   * Потоки создаются через `thread::spawn`.
   * `.join()` позволяет дождаться завершения потока.
   * Безопасна благодаря системе владения и заимствования Rust.

2. **Передача данных между потоками**:

   * Используются каналы из `std::sync::mpsc`.
   * `Sender` и `Receiver` обеспечивают безопасный обмен.
   * Можно использовать клон `Sender` для множественных производителей.

3. **Асинхронность (`async` / `await`)**:

   * Позволяет эффективно выполнять I/O без блокировки.
   * Асинхронные функции возвращают `Future`.
   * Для запуска async-кода нужен executor (`tokio`, `futures`, `async-std`).

🚀 **Ресурсы:**

* The Rust Book — [Глава 16: Многопоточность](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
* The Rust Async Book — [Async Rust](https://rust-lang.github.io/async-book/)
* Tokio runtime — [https://tokio.rs](https://tokio.rs)
* YouTube: Гоша Дударь — [Изучение Rust (видео)](https://www.youtube.com/watch?v=sTr-MOsZwh4&list=PL0lO_mIqDDFU_3UaxCF6p98ELxXpAyHpW&index=10)

---

Если хочешь, могу оформить PDF или продолжить с лекциями по `Mutex`, `Arc`, `RwLock` или `Future` более подробно.

