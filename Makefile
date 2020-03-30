default: ci

test:
	RUSTFLAGS='-F warnings' RUST_BACKTRACE=full cargo test --all

clippy:
	RUSTFLAGS='-F warnings' cargo clippy --all --tests

fmt:
	cargo fmt --all -- --check

ci: fmt clippy test

.PHONY: test clippy fmt ci
