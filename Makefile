format: 
	cargo fmt --all
	cargo clippy --all-features -- -D warnings

check_format:
	cargo fmt --all -- --check
	cargo clippy --all-features -- -D warnings

test:
	cargo test --all-features

changelog:
	cargo changelog --write tevec

try_publish_patch:
	cargo release version patch
	cargo release publish

publish_patch:
	cargo release version patch --execute
	cargo release publish --execute