import rust_fibonacci
import matplotlib.pyplot as plt

# Генерируем данные
n = 20
fib_seq = rust_fibonacci.fibonacci_sequence(n)

# Строим график
plt.figure(figsize=(10, 5))
plt.plot(range(n), fib_seq, 'o-', label='Fibonacci')
plt.title(f'Первые {n} чисел Фибоначчи (Rust + Python)')
plt.xlabel('Номер')
plt.ylabel('Значение')
plt.grid(True)
plt.legend()
plt.savefig('fibonacci_plot.png')
plt.show()