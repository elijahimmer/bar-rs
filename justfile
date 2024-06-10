alias r := run
alias n := nix

run:
	cargo clippy
	cargo build

	-killall bar-rs .bar-rs-wrapped

	RUST_LOG=trace ./target/debug/bar-rs 

nix:
	nix flake check --all-systems
