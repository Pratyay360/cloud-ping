# Justfile for Cloud Ping RS
# Install just: https://github.com/casey/just

# Default recipe to display help
default:
    @just --list

# Build the project
build:
    cargo build

# Build in release mode
build-release:
    cargo build --release

# Run the project
run *ARGS:
    cargo run -- {{ARGS}}

# Run in release mode
run-release *ARGS:
    cargo run --release -- {{ARGS}}

# Run all tests
test:
    cargo test --all-features --all-targets

# Run tests with output
test-verbose:
    cargo test --all-features --all-targets -- --nocapture

# Run specific test
test-one TEST:
    cargo test {{TEST}} -- --nocapture

# Run benchmarks
bench:
    cargo bench

# Run specific benchmark
bench-one BENCH:
    cargo bench {{BENCH}}

# Format code
fmt:
    cargo fmt --all

# Check formatting
fmt-check:
    cargo fmt --all -- --check

# Run clippy
clippy:
    cargo clippy --all-targets --all-features

# Run clippy with strict settings
clippy-strict:
    cargo clippy --all-targets --all-features -- -D warnings

# Fix clippy warnings automatically
clippy-fix:
    cargo clippy --fix --all-targets --all-features

# Check code without building
check:
    cargo check --all-targets --all-features

# Build documentation
doc:
    cargo doc --no-deps --all-features --open

# Build documentation without opening
doc-build:
    cargo doc --no-deps --all-features

# Clean build artifacts
clean:
    cargo clean

# Run all checks (format, clippy, test)
ci: fmt-check clippy-strict test
    @echo "All checks passed!"

# Install development tools
install-tools:
    cargo install cargo-watch
    cargo install cargo-tarpaulin
    cargo install cargo-audit
    cargo install cargo-outdated
    cargo install cargo-tree
    cargo install flamegraph

# Watch for changes and run tests
watch:
    cargo watch -x test

# Watch for changes and run specific command
watch-cmd CMD:
    cargo watch -x "{{CMD}}"

# Check for security vulnerabilities
audit:
    cargo audit

# Check for outdated dependencies
outdated:
    cargo outdated

# Update dependencies
update:
    cargo update

# Show dependency tree
tree:
    cargo tree

# Run code coverage
coverage:
    cargo tarpaulin --out Html --output-dir coverage

# Profile the application
profile *ARGS:
    cargo flamegraph -- {{ARGS}}

# Quick benchmark test
quick:
    cargo run --release -- quick

# Comprehensive benchmark
benchmark:
    cargo run --release -- benchmark

# List all regions
list:
    cargo run --release -- list

# Test specific URL
test-url URL:
    cargo run --release -- test {{URL}}

# Validate configuration
validate:
    cargo run --release -- validate

# Generate config file
gen-config:
    cargo run --release -- config --output config.toml

# Show current config
show-config:
    cargo run --release -- config --show

# Pre-commit checks
pre-commit: fmt clippy test
    @echo "Pre-commit checks passed!"

# Prepare for release
release-prep VERSION:
    @echo "Preparing release {{VERSION}}"
    @echo "1. Update version in Cargo.toml"
    @echo "2. Update CHANGELOG.md"
    @echo "3. Run: just ci"
    @echo "4. Commit changes"
    @echo "5. Tag: git tag -a v{{VERSION}} -m 'Release {{VERSION}}'"
    @echo "6. Push: git push origin v{{VERSION}}"

# Build for all targets
build-all:
    cargo build --release --target x86_64-unknown-linux-gnu
    cargo build --release --target x86_64-unknown-linux-musl
    cargo build --release --target x86_64-apple-darwin
    cargo build --release --target x86_64-pc-windows-msvc

# Install locally
install:
    cargo install --path .

# Uninstall
uninstall:
    cargo uninstall cloud-ping-rs

# Run example
example NAME:
    cargo run --example {{NAME}}

# List examples
examples:
    @ls examples/*.rs | xargs -n1 basename | sed 's/.rs$//'

# Generate README from lib.rs docs
readme:
    cargo readme > README.md

# Check binary size
size:
    @ls -lh target/release/cloud-ping-rs 2>/dev/null || echo "Build release first: just build-release"

# Strip binary
strip:
    strip target/release/cloud-ping-rs

# Analyze binary
bloat:
    cargo bloat --release

# Show compilation time
timings:
    cargo build --release --timings

# Run with verbose logging
run-verbose *ARGS:
    RUST_LOG=debug cargo run -- --verbose {{ARGS}}

# Run with trace logging
run-trace *ARGS:
    RUST_LOG=trace cargo run -- --verbose {{ARGS}}
