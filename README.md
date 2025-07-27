# 🦀 Rust — Быстрый и Безопасный Язык Программирования

[![Rust Version](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/serde.svg)](https://crates.io/)

**Rust** — это язык системного программирования, который сочетает **высокую производительность** с **гарантиями безопасности памяти**. Идеален для системного ПО, веб-серверов, игр и даже встраиваемых систем!

## 🚀 Быстрый Старт

### Установка Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Первая программа
Создайте файл main.rs:

fn main() {
    println!("Привет, мир! 🦀");
}
Запустите:

bash
rustc main.rs && ./main
📖 Основные Концепции
1. Ownership (Владение)

let s1 = String::from("hello");
let s2 = s1; // s1 больше не действительна!
2. Заимствование (Borrowing)

fn print_len(s: &String) {
    println!("Длина: {}", s.len());
}

let s = String::from("Rust");
print_len(&s); // Передаём ссылку
3. Структуры и Перечисления

struct Point {
    x: i32,
    y: i32,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
🔧 Популярные Крейты (Библиотеки)
Крейт	Описание
serde	Сериализация/десериализация (JSON, YAML)
tokio	Асинхронный runtime
actix-web	Веб-фреймворк
clap	Парсинг аргументов командной строки
📊 Пример: HTTP-Сервер на Actix

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from Rust! 🦀")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
📚 Полезные Ресурсы

Официальная документация "https://doc.rust-lang.org/book/"

Rust by Example "https://doc.rust-lang.org/rust-by-example/"

Крейты на crates.io "https://crates.io/"

🛠 Как Собрать Проект?

cargo build  # Сборка
cargo run    # Запуск
cargo test   # Тесты