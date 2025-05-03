# Лекция: Язык программирования Rust  

## **1. Начало**  
Rust — это современный, безопасный и производительный язык программирования, сочетающий низкоуровневый контроль (как C/C++) с высокоуровневыми абстракциями.  
**Основные принципы Rust:**  
- Безопасность памяти без сборщика мусора.  
- Параллелизм без гонок данных.  
- Высокая производительность.  

## **2. История языка**  
- **2006:** Начало разработки в Mozilla Research (Грейдон Хор).  
- **2010:** Первый публичный релиз.  
- **2015:** Стабильная версия 1.0.  
- **2021:** Rust Foundation (поддержка от Google, Microsoft, AWS и др.).  
- **Сейчас:** Один из самых любимых языков (по опросам Stack Overflow).  

## **3. Где применяется**  
- **Системное программирование** (ОС, драйверы).  
- **Веб-ассемблер** (Wasm) — фронтенд (Yew, Leptos).  
- **Криптография и блокчейн** (Solana, Polkadot).  
- **Инструменты разработки** (rustc, cargo).  
- **GameDev** (движки Bevy, Amethyst).  

## **4. [Компиляция](https://doc.rust-lang.ru/book/ch01-02-hello-world.html)**  
- **AOT-компиляция** (ahead-of-time) — код компилируется в бинарник.  
- **Статическая типизация** — проверки на этапе компиляции.  
- **Нулевая стоимость абстракций** — оптимизации как в C++.  

Пример:  
```rust
fn main() {
    println!("Hello, Rust!");
}
```
Компиляция:  
```sh
rustc main.rs  # создаёт исполняемый файл
./main        # запуск
```

## **5. [Установка](https://doc.rust-lang.ru/book/ch01-01-installation.html)**  
1. **Официальный установщик:**  
   ```sh
   $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
   ```  
2. **Проверка:**  
   ```sh
   rustc --version
   cargo --version
   ```  
3. **Обновление:**  
   ```sh
   rustup update
   ```  

## **6. [Первый проект](https://doc.rust-lang.ru/book/ch01-03-hello-cargo.html)**  
Создадим проект через **Cargo** (менеджер пакетов Rust):  
```sh
cargo new hello_rust
cd hello_rust
```  
Структура:  
```
hello_rust/
├── Cargo.toml  # конфигурация
└── src/
    └── main.rs # код
```  

Запуск:  
```sh
cargo run  # компиляция + запуск
```  

## **7. Итог**  
- **Плюсы Rust:** безопасность, скорость, сообщество.  
- **Минусы:** кривая обучения (ownership, borrowing).  
- **Перспективы:** рост в embedded, WASM, cloud.  

**Дальше:** Изучайте `ownership`, `traits`, `async/await`!  

🚀 **Ресурсы:**  
- Книга [The Rust Programming Language](https://doc.rust-lang.org/book/)  
- Практика на [Rustlings](https://github.com/rust-lang/rustlings)  
- Изучение Rust с Гоша Дударь [Видео](https://www.youtube.com/watch?v=E9owuwSUvLk&list=PL0lO_mIqDDFU_3UaxCF6p98ELxXpAyHpW&index=1)

--- 