.POSIX:
.PHONY: default dev

default: target/release/sway-dynamic

target/release/sway-dynamic: Cargo.toml Cargo.lock src/
	cargo build --release

dev:
	cargo run
