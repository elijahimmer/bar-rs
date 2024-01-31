run:
	cargo clippy --release
	cargo build --release

	-killall bar-rs

	export RUST_LOG=trace; cargo run --release
