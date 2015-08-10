all: play_icfp2015

target/release/solve-davar : Cargo.toml run.py solutions src/davar.rs src/in_out.rs src/main.rs src/opts.rs src/simulate.rs src/solver.rs
	cargo build --release && cargo doc

play_icfp2015 : target/release/solve-davar
	cp target/release/solve-davar play_icfp2015

dist:
	git archive -o xiphon.tar.gz HEAD
