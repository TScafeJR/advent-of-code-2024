.PHONY: fmt
fmt:
	@cargo fmt

.PHONY: fix
fix: fmt

.PHONY: gen-day
gen-day:
	@./scripts/gen-day

include makefiles/*.mk
