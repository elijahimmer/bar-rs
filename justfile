run:
	cargo clippy --release
	cargo build --release

	-killall bar-rs

	cargo run --release
