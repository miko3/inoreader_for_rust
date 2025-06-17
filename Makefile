# Inoreader for Rust - Development Makefile

.DEFAULT_GOAL := help
.PHONY: help build test check audit clean install format lint security deps-check all

# Colors for output
RED=\033[0;31m
GREEN=\033[0;32m
YELLOW=\033[1;33m
BLUE=\033[0;34m
NC=\033[0m # No Color

help: ## Show this help message
	@echo "$(BLUE)Inoreader for Rust - Development Commands$(NC)"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "$(GREEN)%-20s$(NC) %s\n", $$1, $$2}'

install: ## Install required tools for development
	@echo "$(YELLOW)Installing development tools...$(NC)"
	cargo install cargo-audit
	cargo install cargo-deny
	cargo install cargo-outdated
	cargo install cargo-watch
	cargo install cargo-tarpaulin
	cargo install cargo-nextest
	rustup component add clippy rustfmt
	@echo "$(GREEN)✅ Development tools installed$(NC)"

build: ## Build the project
	@echo "$(YELLOW)Building project...$(NC)"
	cargo build
	@echo "$(GREEN)✅ Build completed$(NC)"

build-release: ## Build the project in release mode
	@echo "$(YELLOW)Building project (release)...$(NC)"
	cargo build --release
	@echo "$(GREEN)✅ Release build completed$(NC)"

test: ## Run tests
	@echo "$(YELLOW)Running tests...$(NC)"
	cargo test
	@echo "$(GREEN)✅ Tests completed$(NC)"

test-verbose: ## Run tests with verbose output
	@echo "$(YELLOW)Running tests (verbose)...$(NC)"
	cargo test -- --nocapture
	@echo "$(GREEN)✅ Verbose tests completed$(NC)"

test-watch: ## Run tests in watch mode (TDD)
	@echo "$(YELLOW)Starting TDD watch mode...$(NC)"
	@command -v cargo-watch >/dev/null 2>&1 || { echo "$(RED)❌ cargo-watch not found. Run 'make install' first$(NC)"; exit 1; }
	cargo watch -x test

tdd: test-watch ## Alias for TDD watch mode

test-unit: ## Run unit tests only
	@echo "$(YELLOW)Running unit tests...$(NC)"
	cargo test --lib
	@echo "$(GREEN)✅ Unit tests completed$(NC)"

test-integration: ## Run integration tests only
	@echo "$(YELLOW)Running integration tests...$(NC)"
	cargo test --test '*'
	@echo "$(GREEN)✅ Integration tests completed$(NC)"

test-coverage: ## Generate test coverage report
	@echo "$(YELLOW)Generating coverage report...$(NC)"
	@command -v cargo-tarpaulin >/dev/null 2>&1 || { echo "$(RED)❌ cargo-tarpaulin not found. Run 'make install' first$(NC)"; exit 1; }
	cargo tarpaulin --out html --output-dir coverage
	@echo "$(GREEN)✅ Coverage report generated in coverage/$(NC)"

test-nextest: ## Run tests with nextest (faster)
	@echo "$(YELLOW)Running tests with nextest...$(NC)"
	@command -v cargo-nextest >/dev/null 2>&1 || { echo "$(RED)❌ cargo-nextest not found. Run 'make install' first$(NC)"; exit 1; }
	cargo nextest run
	@echo "$(GREEN)✅ Nextest completed$(NC)"

check: ## Run cargo check
	@echo "$(YELLOW)Running cargo check...$(NC)"
	cargo check
	@echo "$(GREEN)✅ Check completed$(NC)"

format: ## Format code
	@echo "$(YELLOW)Formatting code...$(NC)"
	cargo fmt --all
	@echo "$(GREEN)✅ Code formatted$(NC)"

format-check: ## Check code formatting
	@echo "$(YELLOW)Checking code formatting...$(NC)"
	cargo fmt --all -- --check
	@echo "$(GREEN)✅ Format check completed$(NC)"

lint: ## Run clippy lints
	@echo "$(YELLOW)Running clippy...$(NC)"
	cargo clippy --all-targets --all-features -- -D warnings
	@echo "$(GREEN)✅ Linting completed$(NC)"

audit: ## Run security audit
	@echo "$(YELLOW)Running security audit...$(NC)"
	@command -v cargo-audit >/dev/null 2>&1 || { echo "$(RED)❌ cargo-audit not found. Run 'make install' first$(NC)"; exit 1; }
	cargo audit
	@echo "$(GREEN)✅ Security audit completed$(NC)"

deps-check: ## Check for dependency issues
	@echo "$(YELLOW)Checking dependencies...$(NC)"
	@command -v cargo-deny >/dev/null 2>&1 || { echo "$(RED)❌ cargo-deny not found. Run 'make install' first$(NC)"; exit 1; }
	cargo deny check
	@echo "$(GREEN)✅ Dependency check completed$(NC)"

deps-update: ## Check for outdated dependencies
	@echo "$(YELLOW)Checking for outdated dependencies...$(NC)"
	cargo update --dry-run
	@echo "$(GREEN)✅ Dependency update check completed$(NC)"

security: audit deps-check ## Run all security checks

quality: format-check lint test ## Run all quality checks

ci: quality security ## Run all CI checks (quality + security)

clean: ## Clean build artifacts
	@echo "$(YELLOW)Cleaning build artifacts...$(NC)"
	cargo clean
	@echo "$(GREEN)✅ Clean completed$(NC)"

run-setup: ## Run setup command
	@echo "$(YELLOW)Running setup...$(NC)"
	cargo run -- setup

run-fetch: ## Run fetch_stream command
	@echo "$(YELLOW)Running fetch_stream...$(NC)"
	cargo run -- fetch_stream

docs: ## Generate and open documentation
	@echo "$(YELLOW)Generating documentation...$(NC)"
	cargo doc --open
	@echo "$(GREEN)✅ Documentation generated$(NC)"

benchmark: ## Run benchmarks (if any)
	@echo "$(YELLOW)Running benchmarks...$(NC)"
	cargo bench
	@echo "$(GREEN)✅ Benchmarks completed$(NC)"

all: clean ci build-release ## Run complete build pipeline

# Advanced targets
pre-commit: format lint test ## Run pre-commit checks

pre-push: ci build-release ## Run pre-push checks

release-check: ## Check if ready for release
	@echo "$(YELLOW)Performing release readiness check...$(NC)"
	@$(MAKE) --no-print-directory format-check
	@$(MAKE) --no-print-directory lint
	@$(MAKE) --no-print-directory test
	@$(MAKE) --no-print-directory security
	@$(MAKE) --no-print-directory build-release
	@echo "$(GREEN)✅ Release readiness check completed$(NC)"

dev-setup: install ## Setup development environment
	@echo "$(YELLOW)Setting up development environment...$(NC)"
	@if [ ! -f .env ]; then \
		echo "$(YELLOW)Creating .env file from .env.example...$(NC)"; \
		cp .env.example .env; \
		echo "$(YELLOW)Please edit .env file with your API credentials$(NC)"; \
	fi
	@echo "$(GREEN)✅ Development environment setup completed$(NC)"

# Docker targets (if needed)
docker-build: ## Build Docker image
	@echo "$(YELLOW)Building Docker image...$(NC)"
	docker build -t inoreader-rust .
	@echo "$(GREEN)✅ Docker image built$(NC)"

docker-run: ## Run in Docker container
	@echo "$(YELLOW)Running in Docker container...$(NC)"
	docker run --rm -it inoreader-rust
	@echo "$(GREEN)✅ Docker run completed$(NC)"

# Debugging targets
debug: ## Build and run with debug info
	@echo "$(YELLOW)Building and running with debug info...$(NC)"
	RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- fetch_stream

# Performance targets
flamegraph: ## Generate flamegraph (requires cargo-flamegraph)
	@echo "$(YELLOW)Generating flamegraph...$(NC)"
	@command -v cargo-flamegraph >/dev/null 2>&1 || { echo "$(RED)❌ cargo-flamegraph not found. Install with: cargo install flamegraph$(NC)"; exit 1; }
	cargo flamegraph --bin inoreader_house_cleaning -- fetch_stream