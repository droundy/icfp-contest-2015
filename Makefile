all: target/release/solve-davar

target/debug/solve-davar target/release/solve-davar : Cargo.toml display_results.py parse.py potential-phrases solutions src src/bin src/davar.rs src/in_out.rs src/main.rs src/opts.rs src/simulate.rs src/solver.rs
	cargo build --release && cargo build && cargo doc && RUST_BACKTRACE=1 cargo test

