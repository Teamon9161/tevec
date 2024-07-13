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

check_patch:
	cargo release version patch

check_publish:
	cargo release publish

patch:
	cargo release version patch --execute

publish:
	cargo release publish --execute