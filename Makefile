.PHONY: fmt
fmt:
	@cargo fmt

.PHONY: fix
fix: fmt

include makefiles/*.mk
