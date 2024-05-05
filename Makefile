.PHONY: help
help: ## Show this help
	@egrep -h '\s##\s' $(MAKEFILE_LIST) |  awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'


DEBUG ?= true
ifeq ($(DEBUG), true)
	RELEASE :=
else ifeq ($(DEBUG),false)
	RELEASE :=--release
endif
TARGET ?=
ifneq ($(TARGET),)
	TARGET := --target $(TARGET)
endif
build: ## Build the project
	@cargo build $(RELEASE)

formatter-run: ## Run formatter
	@cargo fmt -- --check
formatter-fix: ## Check formatter
	@cargo fmt

linter-run: ## Run linter
	@cargo clippy
linter-fix: ## Fix linter errors if possible
	@cargo clippy --fix --allow-dirty --allow-staged

tests-run: ## Run tests
	@cargo test
