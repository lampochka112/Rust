use wasm_bindgen::prelude::*;

/// Функция для сложения двух чисел
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Генерация n-го числа Фибоначчи
#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 | 1 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/// Обработка строки (передача данных между Rust и JS)
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! 🦀", name)
}