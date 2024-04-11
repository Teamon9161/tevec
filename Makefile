format: 
	cargo fmt
	cargo clippy --all-features -- -D warnings

test:
	cargo test --all-features