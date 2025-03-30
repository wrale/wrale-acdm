.PHONY: all build test lint clean doc install help check run update-deps prepare format trim-whitespace

# Default target
all: build test

# Prepare target for pre-commit checks
prepare: format lint check test doc
	@echo "All checks passed and code formatted!"

# Help target
help:
	@echo "Wrale Agnostic Content Dependency Manager (acdm) Makefile"
	@echo ""
	@echo "Usage:"
	@echo "  make              Build and run tests"
	@echo "  make build        Build the project"
	@echo "  make check        Run cargo check"
	@echo "  make test         Run all tests"
	@echo "  make integration  Run integration tests only"
	@echo "  make unit         Run unit tests only"
	@echo "  make lint         Run linters (clippy)"
	@echo "  make format       Format code with rustfmt and remove trailing whitespace"
	@echo "  make trim-whitespace Remove trailing whitespace from text files"
	@echo "  make clean        Clean build artifacts"
	@echo "  make doc          Generate documentation"
	@echo "  make install      Install the binary to the system"
	@echo "  make run          Run the application (use ARGS=\"<args>\" to pass arguments)"
	@echo "  make update-deps  Update dependencies in Cargo.toml"
	@echo "  make prepare      Format code and run all checks (pre-commit)"
	@echo ""
	@echo "Environment variables:"
	@echo "  ARGS              Arguments to pass to the application when using 'make run'"
	@echo "  RUST_LOG          Set log level (e.g. debug, info, warn, error)"

# Build target
build:
	cargo build

# Check target
check:
	cargo check

# Test targets
test:
	cargo test -- --nocapture

# Run only integration tests
integration:
	cargo test --test integration -- --nocapture

# Run only unit tests
unit:
	cargo test --lib -- --nocapture

# Linting
lint:
	cargo clippy -- -D warnings

# Remove trailing whitespace from text-based files
trim-whitespace:
	@echo "Removing trailing whitespace..."
	@find . -type f \
		-not -path "*/target/*" \
		-not -path "*/\.*" \
		-not -path "*/\*.git*" \
		\( -name "*.md" -o -name "*.txt" -o -name "*.toml" -o -name "*.yml" -o -name "*.yaml" \) \
		-exec sed -i '' -E 's/[[:space:]]+$$//' {} \;
	@echo "Trailing whitespace removed."

# Format code
format: trim-whitespace
	cargo fmt --all

# Clean
clean:
	cargo clean

# Generate documentation
doc:
	cargo doc --no-deps

# Install
install:
	cargo install --path .

# Run with arguments
run:
	cargo run -- $(ARGS)

# Update dependencies
update-deps:
	cargo update
