all: target/debug/solve-davar

target/debug/solve-davar target/release/solve-davar : Cargo.toml Makefile src src/davar.rs src/main.rs target/debug
	cargo build --release && cargo build && cargo doc && cargo test

