.PHONY: help
help: ## Show this help
	@DESCRIPTION_WIDTH=$$(grep -Eh '^[a-zA-Z0-9\._-]+:.*?##' $(MAKEFILE_LIST) | \
		awk -F ':.*?##' '{ if (length($$1) > max) max = length($$1) } END { print max }'); \
	grep -Eh '^[a-zA-Z0-9\._-]+:.*?##' $(MAKEFILE_LIST) | \
	awk -v width=$$DESCRIPTION_WIDTH 'BEGIN { FS = ":.*?##" } { printf "\033[36m%-" width "s\033[0m %s\n", $$1, $$2 }'

RELEASE_FLAG :=
ifeq ($(RELEASE), true)
	RELEASE_FLAG :=--release
endif
TARGET_FLAG :=
ifneq ($(TARGET),)
	TARGET_FLAG := --target $(TARGET)
endif
build: ## Build the project
	@cargo build $(RELEASE_FLAG) $(TARGET_FLAG)

tests-run: ## Run tests
	@cargo test

formatter-check: ## Check formatting
	@cargo fmt -- --check
formatter-fix: ## Fix formatting
	@cargo fmt

linter-check: ## Check linting
	@cargo clippy
linter-fix: ## Fix linting errors if possible
	@cargo clippy --fix --allow-dirty --allow-staged

spellchecker-check: ## Check spelling
	@typos ./
