# Rust Claude Code Template - Justfile
# Quick development commands for Rust projects

# Default recipe - show available commands
default:
    @just --list

# Development commands
alias d := dev
alias r := run
alias t := test
alias c := check
alias f := fmt
alias l := lint

# === DEVELOPMENT ===

# Run the project in development mode
dev:
    cargo run

# Run the project with hot reloading
watch:
    cargo watch -x run

# Run the project in release mode
run:
    cargo run --release

# Run with all features enabled
run-all:
    cargo run --all-features

# === BUILDING ===

# Build the project
build:
    cargo build

# Build in release mode
build-release:
    cargo build --release

# Build with optimizations for native CPU
build-native:
    RUSTFLAGS="-C target-cpu=native" cargo build --release

# Clean build artifacts
clean:
    cargo clean

# === TESTING ===

# Run all tests
test:
    cargo test

# Run tests with output
test-verbose:
    cargo test -- --nocapture

# Run tests with specific pattern
test-pattern PATTERN:
    cargo test {{PATTERN}}

# Run tests and watch for changes
test-watch:
    cargo watch -x test

# Run tests with coverage (requires cargo-tarpaulin)
coverage:
    cargo tarpaulin --out Html

# Run property-based tests only (if using proptest)
test-prop:
    cargo test prop

# Run integration tests only
test-integration:
    cargo test --test '*'

# Run benchmarks
bench:
    cargo bench

# === CODE QUALITY ===

# Format code
fmt:
    cargo fmt

# Check formatting without making changes
fmt-check:
    cargo fmt --check

# Run clippy linter
lint:
    cargo clippy -- -D warnings

# Run clippy with all targets
lint-all:
    cargo clippy --all-targets --all-features -- -D warnings

# Quick check without building
check:
    cargo check

# Check all targets and features
check-all:
    cargo check --all-targets --all-features

# Fix automatically fixable lints
fix:
    cargo fix --allow-dirty

# === DOCUMENTATION ===

# Generate and open documentation
doc:
    cargo doc --open

# Generate documentation for all dependencies
doc-all:
    cargo doc --all --open

# Check documentation for errors
doc-check:
    cargo doc --no-deps

# === DEPENDENCIES ===

# Update dependencies
update:
    cargo update

# Audit dependencies for security vulnerabilities
audit:
    cargo audit

# Check for outdated dependencies
outdated:
    cargo outdated

# Add a new dependency
add CRATE:
    cargo add {{CRATE}}

# Add a development dependency
add-dev CRATE:
    cargo add --dev {{CRATE}}

# Remove a dependency
remove CRATE:
    cargo remove {{CRATE}}

# === UTILITY ===

# Show project tree structure
tree:
    tree -I 'target|node_modules'

# Show git status
status:
    git status

# Create a new module
new-module NAME:
    mkdir -p src/{{NAME}}
    echo "//! {{NAME}} module" > src/{{NAME}}/mod.rs
    echo "pub mod {{NAME}};" >> src/lib.rs

# Create a new integration test
new-test NAME:
    echo "//! Integration test for {{NAME}}" > tests/{{NAME}}.rs

# Create a new example
new-example NAME:
    echo "//! Example: {{NAME}}" > examples/{{NAME}}.rs

# === CI/CD SIMULATION ===

# Run all CI checks locally
ci: fmt-check lint test doc-check
    @echo "All CI checks passed!"

# Pre-commit hook simulation
pre-commit: fmt lint test
    @echo "Pre-commit checks passed!"

# Full development cycle check
full-check: clean build test lint doc audit
    @echo "Full development cycle completed successfully!"

# === INSTALLATION ===

# Install development tools
install-tools:
    rustup component add rustfmt clippy
    cargo install cargo-watch cargo-tarpaulin cargo-audit cargo-outdated

# Install additional development tools
install-extras:
    cargo install cargo-expand cargo-machete cargo-deny cargo-udeps

# === TEMPLATE SETUP ===

# Initialize a new project from this template
init PROJECT_NAME:
    #!/usr/bin/env bash
    if [ -f Cargo.toml ]; then
        echo "Cargo.toml already exists. Skipping initialization."
    else
        echo "Initializing new Rust project: {{PROJECT_NAME}}"
        cargo init --name {{PROJECT_NAME}}
        mkdir -p src/modules tests examples benches docs
        echo "pub mod modules;" > src/lib.rs
        echo "Project initialized! Edit Cargo.toml to customize dependencies."
    fi

# Clean up template for new project
cleanup-template:
    rm -rf .git
    git init
    @echo "Template cleaned up. Ready for your new project!"

# === RELEASE ===

# Prepare for release (dry run)
release-check:
    cargo publish --dry-run

# Create a new release (requires manual version bump)
release:
    cargo publish

# === ADVANCED ===

# Profile the application
profile:
    cargo build --release
    perf record --call-graph=dwarf ./target/release/$(basename $(pwd))
    perf report

# Expand macros for debugging
expand:
    cargo expand

# Find unused dependencies
unused-deps:
    cargo machete

# Security-focused dependency check
security-check:
    cargo deny check

# Find duplicate dependencies
duplicate-deps:
    cargo tree --duplicates

# === HELP ===

# Show detailed help for cargo commands
help:
    @echo "Cargo commands reference:"
    @echo "  cargo run      - Run the project"
    @echo "  cargo test     - Run tests"
    @echo "  cargo build    - Build the project"
    @echo "  cargo fmt      - Format code"
    @echo "  cargo clippy   - Run linter"
    @echo "  cargo check    - Quick syntax check"
    @echo "  cargo doc      - Generate documentation"
    @echo ""
    @echo "Use 'just <command>' for convenience aliases!"