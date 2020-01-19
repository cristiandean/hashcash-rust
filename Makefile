setup:
	@echo "Make sure you've rust and cargo installed "
	@sleep 3
	rustup toolchain install nightly
	rustup update
	rustup target add wasm32-unknown-unknown --toolchain nightly
	cargo install --git https://github.com/alexcrichton/wasm-gc

build:
	cargo  +nightly build --target wasm32-unknown-unknown --release
	wasm-gc ./target/wasm32-unknown-unknown/release/hashcash.wasm ./public/hashcash.wasm

run:
	python -m SimpleHTTPServer 8000

all: build run
