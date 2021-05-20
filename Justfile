test:
	cargo test -- --test-threads 1

f:
	rustfmt $(shell find src -name "*.rs" -type f)
