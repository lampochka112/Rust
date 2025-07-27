use pyo3::prelude::*;

/// Быстрое вычисление n-го числа Фибоначчи (итеративный метод)
#[pyfunction]
fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let (mut a, mut b) = (0, 1);
    for _ in 1..n {
        (a, b) = (b, a + b);
    }
    b
}

/// Возвращает вектор первых N чисел Фибоначчи
#[pyfunction]
fn fibonacci_sequence(n: usize) -> Vec<u64> {
    let mut seq = Vec::with_capacity(n);
    if n >= 1 {
        seq.push(0);
    }
    if n >= 2 {
        seq.push(1);
    }
    for i in 2..n {
        let next = seq[i-1] + seq[i-2];
        seq.push(next);
    }
    seq
}

/// Регистрация модуля для Python
#[pymodule]
fn rust_fibonacci(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?);
    m.add_function(wrap_pyfunction!(fibonacci_sequence, m)?);
    Ok(())
}