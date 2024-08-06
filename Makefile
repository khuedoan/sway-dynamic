.POSIX:
.PHONY: default dev fmt lint

default: fmt lint target/release/sway-dynamic

target/release/sway-dynamic: Cargo.toml Cargo.lock src/
	cargo build --release

dev:
	cargo run

fmt:
	cargo fmt

lint:
	cargo clippy -- --deny warnings
