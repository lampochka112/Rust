import init, { add, fibonacci, greet } from './pkg/rust_wasm_js.js';

async function run() {
    // Инициализация WASM
    await init();

    // Тестируем функции
    document.getElementById('btn-add').addEventListener('click', () => {
        const result = add(2, 3);
        document.getElementById('output').textContent = `add(2, 3) = ${result}`;
    });

    document.getElementById('btn-fib').addEventListener('click', () => {
        const result = fibonacci(10);
        document.getElementById('output').textContent = `fibonacci(10) = ${result}`;
    });

    document.getElementById('btn-greet').addEventListener('click', () => {
        const result = greet("World");
        document.getElementById('output').textContent = result;
    });
}

run();