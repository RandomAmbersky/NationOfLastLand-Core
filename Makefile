# Makefile для сборки проекта NationOfLastLand-Core на Rust

.PHONY: all build release clean run test check wasm wasm-release example-wasm

# По умолчанию сборка в режиме debug
all: build

# Сборка в режиме debug
build:
	cargo build

# Сборка в режиме release
release:
	cargo build --release

# Сборка для WebAssembly (debug)
wasm:
	cargo build --target wasm32-unknown-unknown

# Сборка для WebAssembly (release)
wasm-release:
	cargo build --target wasm32-unknown-unknown --release

# Сборка example-wasm проекта
example-wasm:
	cd example-wasm && wasm-pack build --target web --out-dir pkg

# Сборка example-wasm проекта (release)
example-wasm-release:
	cd example-wasm && wasm-pack build --target web --out-dir pkg --release

# Служба example-wasm (требует python для простого HTTP сервера)
example-wasm-serve:
	cd example-wasm && python3 -m http.server 8091

# Очистка
clean:
	cargo clean

# Запуск
run:
	cargo run

# Тестирование
test:
	cargo test

# Проверка кода без сборки
check:
	cargo check
