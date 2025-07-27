#include <iostream>
#include <vector>

// Объявляем функции из Rust
extern "C" {
    int fibonacci(int n);
    double sum_array(const double* arr, int len);
}

int main() {
    // Пример 1: Число Фибоначчи
    int n = 10;
    std::cout << "Fibonacci(" << n << ") = " << fibonacci(n) << std::endl;

    // Пример 2: Сумма массива
    std::vector<double> arr = {1.5, 2.3, 3.7, 4.1};
    double sum = sum_array(arr.data(), arr.size());
    std::cout << "Sum of array = " << sum << std::endl;

    return 0;
}