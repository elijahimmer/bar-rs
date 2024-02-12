alias r := run
alias n := nix

run:
	cargo clippy --release
	cargo build --release --features dynamic_css

	-killall bar-rs .bar-rs-wrapped

	export RUST_LOG=trace; ./target/release/bar-rs 

nix:
	nix flake check --all-systems
