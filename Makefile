all: target/debug/solve-davar

target/debug/solve-davar target/release/solve-davar : Cargo.toml src src/davar.rs src/json.rs src/main.rs src/simulate.rs
	cargo build --release && cargo build && cargo doc && cargo test

