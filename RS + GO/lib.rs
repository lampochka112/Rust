use libc::{c_int, c_double};

/// Сложение двух чисел (для демонстрации)
#[no_mangle]
pub extern "C" fn add(a: c_int, b: c_int) -> c_int {
    a + b
}

/// Расчёт среднего значения массива
#[no_mangle]
pub extern "C" fn average(arr: *const c_double, len: c_int) -> c_double {
    let mut sum = 0.0;
    unsafe {
        for i in 0..len {
            sum += *arr.offset(i as isize);
        }
    }
    sum / len as c_double
}