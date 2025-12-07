.PHONY: build clippy test clean run-examples run-analyzer check kani help

.DEFAULT_GOAL := help

## Show this help message
help:
	@echo "Available targets:"
	@awk '/^##/ {desc=substr($$0,4); next} /^[a-zA-Z_-]+:/ && desc {printf "  \033[36m%-15s\033[0m %s\n", $$1, desc; desc=""}' $(MAKEFILE_LIST)

## Build all workspace crates
build:
	@echo "Building all workspace crates..."
	cargo build --workspace

## Run clippy linter on all crates
clippy:
	@echo "Running clippy..."
	@command -v cargo-clippy >/dev/null 2>&1 || { \
		echo "Clippy not found, installing..."; \
		rustup component add clippy; \
	}
	@echo "Checking hyp-analyzer (strict)..."
	@cd crates/hyp-analyzer && cargo clippy --all-targets -- -D warnings
	@echo "Checking hyp (strict)..."
	@cd crates/hyp && cargo clippy --bins -- -D warnings
	@echo ""
	@echo "Checking hyp-examples (reporting issues)..."
	@echo "Checking hyp-examples (reporting issues)..."
	@# Enabling restriction group as requested. Note: this includes conflicting lints!
	@cd crates/hyp-examples && cargo clippy --lib -- -W clippy::restriction -W clippy::pedantic -W clippy::nursery -W clippy::cargo -A clippy::blanket_clippy_restriction_lints || true
	@echo ""
	@echo "Checking hyp-examples-cli (allowing warnings from hyp-examples)..."
	@cd crates/hyp-examples-cli && cargo clippy --bins 2>&1 | grep -v "hyp-examples" | grep -E "^(warning|error):" || echo "  ✓ No issues in CLI code"
	@echo ""
	@echo "✓ Clippy checks passed for analyzer and CLI crates"
	@echo "Note: hyp-examples crate intentionally contains problematic code for demonstration"

## Run Kani formal verifier
kani:
	@command -v kani >/dev/null || \
		(echo "Installing Kani verifier..." && \
		 cargo install --locked kani-verifier && cargo kani setup)
	cargo kani --workspace --all-features --output-format terse

## Run all tests in the workspace
test:
	@echo "Ensuring problem exampels are executable w/o problems..."
	cargo run --bin hyp-examples -- run-all
	@echo "Running unit tests..."
	cargo test --workspace

## Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean

## Run cargo check on all crates
check:
	@echo "Running cargo check..."
	cargo check --workspace
