default: help

HELP_FORMAT="    \033[36m%-25s\033[0m %s\n"
.PHONY: help
help: ## Display this usage information (default)
	@echo "Valid targets:"
	@grep -E '^[^ ]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		sort | \
		awk 'BEGIN {FS = ":.*?## "}; \
			{printf $(HELP_FORMAT), $$1, $$2}'
	@echo ""

.PHONY: lint
lint: ## Lint the nomad-rs-api code
	@echo "==> Linting source code..."
	@cargo clippy --all-targets --all-features -- -Dwarnings
	@cargo fmt --check
	@echo "==> Done"
