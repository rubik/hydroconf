.PHONY: test
test:
	cargo test -- --test-threads 1

.PHONY: f
f:
	rustfmt $(shell find src -name "*.rs" -type f)
