# ClaudeForge CLI

[![CI](https://github.com/iepathos/claudeforge/actions/workflows/ci.yml/badge.svg)](https://github.com/iepathos/claudeforge/actions/workflows/ci.yml)
[![Security](https://github.com/iepathos/claudeforge/actions/workflows/security.yml/badge.svg)](https://github.com/iepathos/claudeforge/actions/workflows/security.yml)
[![Release](https://github.com/iepathos/claudeforge/actions/workflows/release.yml/badge.svg)](https://github.com/iepathos/claudeforge/actions/workflows/release.yml)

A command-line tool that streamlines the creation of new projects optimized for development with Claude Code. ClaudeForge provides a simple interface to scaffold projects in multiple languages using curated templates that include comprehensive AI development guidelines, proper gitignore configurations, and best practices baked into the project structure.

## 🚀 Quick Start

### Installation

#### Quick Install Latest for macOS/Linux (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/iepathos/claudeforge/HEAD/install.sh | bash
```

Or download tarball directly from github releases (replace VERSION and OS/ARCH as needed)
```bash
# Example: claudeforge-0.1.6-x86_64-apple-darwin.tar.gz
curl -LO https://github.com/iepathos/claudeforge/releases/latest/download/claudeforge-VERSION-OS-ARCH.tar.gz
tar -xzf claudeforge-*.tar.gz
sudo mv claudeforge /usr/local/bin/
```

#### Alternative Installation Methods

```bash
# Install from crates.io
cargo install claudeforge

# Or build from source
git clone https://github.com/iepathos/claudeforge.git
cd claudeforge
cargo build --release
```

### Create Your First Project

```bash
# Create a new Rust project
claudeforge new rust my-awesome-project

# Create a Go project in a specific directory
claudeforge new go my-service --directory ~/projects

# Skip confirmation prompts
claudeforge new rust my-project --yes
```

## 📋 Commands

### `new` - Create a new project
```bash
claudeforge new <LANGUAGE> <NAME> [OPTIONS]

# Arguments:
#   <LANGUAGE>  Language template to use (rust, go, python)
#   <NAME>      Project name

# Options:
#   -d, --directory <DIR>  Target directory (defaults to current directory)
#   -y, --yes             Skip interactive prompts
```

### `list` - List available templates
```bash
claudeforge list

# Shows all available templates with descriptions
```

### `update` - Update cached templates
```bash
claudeforge update

# Updates only the templates you've previously used
# Templates are cached locally when you first use them with 'claudeforge new'
```

### `version` - Show version information
```bash
claudeforge version
```

## 🎯 Features

- **Multi-language support**: Currently supports Rust, Go, and Python templates
- **AI-optimized templates**: Pre-configured with CLAUDE.md guidelines
- **Git integration**: Automatically initializes clean git repositories
- **Template customization**: Replaces project placeholders with your values
- **Smart caching**: Templates are cached on first use for faster subsequent project creation
- **Cross-platform**: Works on macOS, Linux, and Windows

## 📁 Project Structure

ClaudeForge creates projects with the following structure:

```
my-project/
├── src/                    # Source code
├── tests/                  # Test files
├── CLAUDE.md              # Claude Code development guidelines
├── .gitignore             # Language-specific gitignore
├── README.md              # Project documentation
├── Cargo.toml             # Rust: Project manifest
├── go.mod                 # Go: Module definition
└── pyproject.toml         # Python: Project configuration
```

## 🛠️ Development

### Prerequisites

- Rust 1.70.0 or later
- Git (for repository operations)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/iepathos/claudeforge.git
cd claudeforge

# Build the project
cargo build --release

# Run tests
cargo test

# Install locally
cargo install --path .
```

### Development Commands

```bash
# Run with just
just fmt      # Format code
just lint     # Run clippy
just test     # Run all tests
just build    # Build release binary

# Or use cargo directly
cargo fmt
cargo clippy -- -D warnings
cargo test
cargo build --release
```

## 🤖 Claude Code Integration

Each generated project includes a comprehensive `CLAUDE.md` file that provides:

- **Architecture guidelines**: Error handling, concurrency patterns, and configuration management
- **Code style standards**: Documentation, logging, and testing requirements
- **Development patterns**: Best practices and anti-patterns specific to each language
- **Example prompts**: How to effectively communicate with Claude for various tasks

### Template Features

**Rust Templates:**
- Pre-configured `Cargo.toml` with common dependencies
- Async/await support with tokio
- Comprehensive error handling with `anyhow`
- Testing setup with unit and integration tests
- CLI argument parsing with `clap`

**Go Templates:**
- Modern Go module structure
- Context-aware error handling
- Structured logging setup
- Testing patterns and examples
- CLI framework integration

**Python Templates:**
- Modern Python project structure with pyproject.toml
- Virtual environment and dependency management
- Comprehensive testing with pytest
- Type hints and mypy configuration
- CLI framework integration with Click

## 📦 Template Registry

ClaudeForge uses a built-in registry system to manage templates. Currently, templates are defined in the source code with the following built-in templates:

- **Rust**: Comprehensive Rust starter template with Claude Code guidelines
- **Go**: Go project template optimized for Claude Code development  
- **Python**: Python project template with Claude Code integration

Templates are cached locally when first used with `claudeforge new` and can be updated from their respective GitHub repositories using `claudeforge update`.

## 🔧 Configuration

### Global Configuration

Create `~/.config/claudeforge/config.toml`:

```toml
[defaults]
author_name = "Your Name"
author_email = "your.email@example.com"
default_directory = "~/projects"

[templates]
cache_directory = "~/.cache/claudeforge"
auto_update = true
update_interval_days = 7
```

### Template Customization

Templates support placeholder replacement:

- `{{PROJECT_NAME}}` - Project name
- `{{AUTHOR_NAME}}` - Author name from git config
- `{{AUTHOR_EMAIL}}` - Author email from git config
- `{{CURRENT_DATE}}` - Current date (YYYY-MM-DD)

## 🚀 Example Usage

### Creating a Rust Web Service

```bash
claudeforge new rust my-web-service
cd my-web-service

# The project is ready for Claude Code development
claude code .
```

### Creating a Go CLI Tool

```bash
claudeforge new go my-cli-tool --directory ~/work/projects
cd ~/work/projects/my-cli-tool

# Start developing immediately
go run main.go
```

### Creating a Python Application

```bash
claudeforge new python my-python-app
cd my-python-app

# Set up virtual environment and start developing
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -e .
```

## 🔍 Troubleshooting

### Common Issues

**Template not found:**
```bash
# List available templates
claudeforge list

# Create a project to cache the template
claudeforge new rust my-project

# Update cached templates
claudeforge update
```

**Git not found:**
```bash
# Install git on your system
# macOS: brew install git
# Ubuntu: sudo apt install git
# Windows: Download from git-scm.com
```

**Permission denied:**
```bash
# Ensure you have write permissions to the target directory
chmod +w target-directory
```

## 🤝 Contributing

We welcome contributions to ClaudeForge! Please see our contribution guidelines:

1. Fork the repository
2. Create a feature branch
3. Make your changes following the guidelines in `CLAUDE.md`
4. Ensure all tests pass: `just test`
5. Submit a pull request

### Adding New Templates

To add support for a new language:

1. Create a template repository with the language structure
2. Add template configuration to `src/template/registry.rs`
3. Update the `Language` enum in `src/cli.rs`
4. Test the template creation process
5. Update documentation

## 📝 License

GPL-3.0 License - see [LICENSE](LICENSE) file for details.

## 🎯 Roadmap

- [ ] Additional language templates (Python, JavaScript, TypeScript)
- [ ] Custom template support from local directories
- [ ] Template versioning and rollback
- [ ] Interactive template selection
- [ ] Plugin system for custom processors
- [ ] Integration with popular project hosting platforms

---

**Happy coding with ClaudeForge! 🔨🤖**

Create better projects faster with AI-optimized templates and get straight to building amazing software.
