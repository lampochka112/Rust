use libc::{c_int, c_double};

/// Быстрое вычисление n-го числа Фибоначчи (экспортируемая функция)
#[no_mangle]
pub extern "C" fn fibonacci(n: c_int) -> c_int {
    if n <= 1 {
        return n;
    }
    let (mut a, mut b) = (0, 1);
    for _ in 2..=n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

/// Сумма элементов массива (для демонстрации работы с памятью)
#[no_mangle]
pub extern "C" fn sum_array(arr: *const c_double, len: c_int) -> c_double {
    let mut sum = 0.0;
    unsafe {
        for i in 0..len {
            sum += *arr.offset(i as isize);
        }
    }
    sum
}