all: play_icfp2015

target/debug/solve-davar target/release/solve-davar : Cargo.toml display_results.py parse.py pop potential-phrases run.py solutions src src/bin src/davar.rs src/in_out.rs src/main.rs src/opts.rs src/simulate.rs src/solver.rs submit_best.py test-phrase.py
	cargo build --release && cargo build && cargo doc && RUST_BACKTRACE=1 cargo test

play_icfp2015 : target/release/solve-davar
	cp target/release/solve-davar play_icfp2015

