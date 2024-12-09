##
## * Rust Specific Targets

## make rust-test; - run rust tests
.PHONY: rust-test
rust-test:
	@cargo test

.PHONY: rust-fmt
rust-fmt:
	@cargo fmt
