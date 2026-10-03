SHELL:=/bin/sh

CARGO := cargo

.PHONY: all

help: ## this help
	@awk 'BEGIN {FS = ":.*?## ";  printf "Usage:\n  make \033[36m<target> \033[0m\n\nTargets:\n"} /^[a-zA-Z0-9_-]+:.*?## / {gsub("\\\\n",sprintf("\n%22c",""), $$2);printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

test: ## Run tests with nextest
	$(CARGO) nextest run

test-verbose: ## Run tests and show output
	$(CARGO) test -- --nocapture

fmt: ## Format code with rustfmt
	$(CARGO) fmt --all --

lint: ## Lint code with clippy
	$(CARGO) clippy -- -D warnings

run: ## Run a specific example (usage: make run EXAMPLE=hello_world)
	$(CARGO) run --release

build: ## Build
	$(CARGO) build

release: ## Build release version
	$(CARGO) build --release

clean: ## Clean build artifacts
	$(CARGO) clean

ci: fmt lint test ## Run all checks for CI

docs: ## Render README.md using glow
	glow README.md
