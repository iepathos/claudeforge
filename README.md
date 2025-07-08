# Rust Claude Code Starter Template

A comprehensive starter template for Rust projects optimized for development with Claude Code. This template provides a solid foundation with best practices, tooling configurations, and development guidelines for building robust Rust applications.

## 🚀 Quick Start

1. **Clone this template**
   ```bash
   git clone https://github.com/yourusername/rust-claude-code.git my-rust-project
   cd my-rust-project
   ```

2. **Initialize your project**
   ```bash
   # Remove template git history
   rm -rf .git
   git init
   
   # Create initial Cargo.toml
   cargo init --name my-project
   ```

3. **Install development dependencies**
   ```bash
   # Install rustfmt and clippy
   rustup component add rustfmt clippy
   
   # Install cargo-watch for development
   cargo install cargo-watch
   
   # Install additional tools (optional)
   cargo install cargo-tarpaulin  # Code coverage
   cargo install cargo-audit      # Security audits
   cargo install cargo-outdated   # Dependency updates
   ```

## 📁 Project Structure

```
my-rust-project/
├── src/                    # Source code
│   ├── main.rs            # Binary entry point
│   ├── lib.rs             # Library entry point
│   └── modules/           # Application modules
├── tests/                 # Integration tests
├── benches/               # Benchmarks
├── examples/              # Usage examples
├── docs/                  # Documentation
├── .gitignore             # Git ignore rules
├── CLAUDE.md              # Claude Code guidelines
├── Cargo.toml             # Project manifest
└── README.md              # This file
```

## 🛠️ Development Workflow

### Running the project
```bash
# Build and run
cargo run

# Run with hot reloading
cargo watch -x run

# Run tests
cargo test

# Run with all features
cargo run --all-features
```

### Code Quality
```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Check without building
cargo check

# Run security audit
cargo audit
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Generate code coverage
cargo tarpaulin --out Html
```

## 🤖 Claude Code Integration

This template includes a comprehensive `CLAUDE.md` file that provides:

- **Architecture guidelines**: Error handling, concurrency patterns, and configuration management
- **Code style standards**: Documentation, logging, and testing requirements
- **Development patterns**: Best practices and anti-patterns specific to Rust
- **Example prompts**: How to effectively communicate with Claude for various tasks

### Key Features for Claude Development

1. **Structured Error Handling**
   - Uses `Result<T, E>` types consistently
   - Includes examples with `anyhow` and `thiserror`

2. **Async/Await Support**
   - Pre-configured for `tokio` runtime
   - Examples for concurrent operations

3. **Comprehensive Testing**
   - Unit test templates
   - Property-based testing with `proptest`
   - Integration test structure

4. **Documentation Standards**
   - Rustdoc comment templates
   - Example-driven documentation

## 📦 Recommended Dependencies

Add these to your `Cargo.toml` as needed:

```toml
[dependencies]
# Async runtime
tokio = { version = "1", features = ["full"] }

# Error handling
anyhow = "1"
thiserror = "1"

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# CLI
clap = { version = "4", features = ["derive"] }

# HTTP client
reqwest = { version = "0.11", features = ["json"] }

[dev-dependencies]
# Testing
proptest = "1"
mockall = "0.11"
criterion = "0.5"
tempfile = "3"
```

## 🔧 Configuration

### Environment Variables

Create a `.env` file for local development:

```env
# Application settings
RUST_LOG=debug
DATABASE_URL=postgresql://localhost/myapp
API_KEY=your_api_key_here
```

### VS Code Settings

Recommended `.vscode/settings.json`:

```json
{
    "rust-analyzer.cargo.features": ["all"],
    "rust-analyzer.checkOnSave.command": "clippy",
    "editor.formatOnSave": true,
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    }
}
```

## 🚀 Building for Production

```bash
# Build release version
cargo build --release

# Run release version
cargo run --release

# Create optimized binary
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

## 📚 Learning Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Async Programming in Rust](https://rust-lang.github.io/async-book/)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/)

## 🤝 Contributing

When contributing to this template:

1. Follow the guidelines in `CLAUDE.md`
2. Ensure all tests pass: `cargo test`
3. Run formatters: `cargo fmt`
4. Check lints: `cargo clippy`
5. Update documentation as needed

## 📝 License

This template is provided as-is for use in your own projects. Customize the license as needed for your specific use case.

---

## 🎯 Next Steps

1. **Customize `Cargo.toml`** with your project details
2. **Update this README** with project-specific information
3. **Review `CLAUDE.md`** for development guidelines
4. **Set up CI/CD** with GitHub Actions or similar
5. **Start building** your Rust application!

Happy coding with Rust and Claude! 🦀🤖