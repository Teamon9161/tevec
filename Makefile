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

check_patch_version:
	cargo release version patch

check_minor_version:
	cargo release version minor

check_major_version:
	cargo release version major

check_publish:
	cargo release publish

patch_version:
	cargo release version patch --execute

minor_version:
	cargo release version minor --execute

major_version:
	cargo release version major --execute

publish:
	cargo release publish --execute