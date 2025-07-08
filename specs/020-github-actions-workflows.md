# Spec 020: GitHub Actions Workflows

## Feature Summary

This feature adds comprehensive GitHub Actions workflows for the ClaudeForge project, providing automated testing, code quality checks, and release management. The workflows will ensure code quality, run tests across multiple platforms, and automate the release process for distributing the CLI tool.

The implementation includes:
- Continuous Integration (CI) workflow for testing and quality checks
- Release workflow for building and distributing binaries
- Security scanning and dependency auditing
- Cross-platform testing (Linux, macOS, Windows)
- Automated changelog generation and GitHub releases

## Goals & Requirements

### Functional Requirements
- **CI Pipeline**: Automated testing on every pull request and push to main
- **Multi-platform Support**: Test and build on Linux, macOS, and Windows
- **Code Quality**: Automated formatting checks, linting, and security scanning
- **Release Automation**: Automated binary builds and GitHub releases
- **Caching**: Efficient build times using dependency caching
- **Security**: Automated dependency auditing and vulnerability scanning

### Non-functional Requirements
- **Performance**: CI runs should complete within 10 minutes
- **Reliability**: Workflows should be stable and not prone to flaky failures
- **Maintainability**: Clear workflow structure and documentation
- **Security**: Secure handling of tokens and release assets

### Success Criteria
- All tests pass on supported platforms
- Code quality checks enforce project standards
- Releases are automatically created with proper versioning
- Security vulnerabilities are detected and reported
- Build artifacts are properly signed and distributed

## API/Interface Design

### Workflow Triggers
```yaml
# CI Workflow triggers
on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

# Release Workflow triggers
on:
  push:
    tags: ['v*']
  workflow_dispatch:
    inputs:
      version:
        description: 'Version to release'
        required: true
        type: string
```

### Workflow Outputs
- **CI**: Test results, code coverage, quality metrics
- **Release**: Binary artifacts, checksums, release notes
- **Security**: Vulnerability reports, dependency updates

### Matrix Strategy
```yaml
strategy:
  matrix:
    os: [ubuntu-latest, macos-latest, windows-latest]
    rust: [stable, beta]
    exclude:
      - os: windows-latest
        rust: beta
```

## File and Package Structure

### Workflow Files
```
.github/
├── workflows/
│   ├── ci.yml                 # Main CI workflow
│   ├── release.yml            # Release workflow
│   ├── security.yml           # Security scanning
│   └── dependabot.yml         # Dependency updates
├── actions/
│   ├── setup-rust/            # Custom Rust setup action
│   └── release-utils/         # Release utilities
└── SECURITY.md                # Security policy
```

### Build Artifacts Structure
```
target/
├── release/
│   ├── claudeforge            # Linux binary
│   ├── claudeforge.exe        # Windows binary
│   └── claudeforge-macos      # macOS binary
└── artifacts/
    ├── checksums.txt          # SHA256 checksums
    ├── signatures.asc         # GPG signatures
    └── release-notes.md       # Generated release notes
```

## Implementation Details

### CI Workflow (`ci.yml`)
```yaml
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    name: Test Suite
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        rust: [stable, beta]
        exclude:
          - os: windows-latest
            rust: beta
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
      with:
        toolchain: ${{ matrix.rust }}
        components: rustfmt, clippy
    
    - name: Cache dependencies
      uses: actions/cache@v4
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target/
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Run tests
      run: cargo test --all-features
    
    - name: Run clippy
      run: cargo clippy --all-targets --all-features -- -D warnings
    
    - name: Check formatting
      run: cargo fmt --all -- --check
```

### Release Workflow (`release.yml`)
```yaml
name: Release

on:
  push:
    tags: ['v*']
  workflow_dispatch:
    inputs:
      version:
        description: 'Version to release'
        required: true
        type: string

jobs:
  create-release:
    name: Create Release
    runs-on: ubuntu-latest
    outputs:
      upload_url: ${{ steps.create_release.outputs.upload_url }}
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Generate release notes
      id: release_notes
      run: |
        # Generate changelog from git history
        echo "## Changes" > release_notes.md
        git log --oneline --format="- %s" $(git describe --tags --abbrev=0)..HEAD >> release_notes.md
    
    - name: Create Release
      id: create_release
      uses: actions/create-release@v1
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      with:
        tag_name: ${{ github.ref }}
        release_name: Release ${{ github.ref }}
        body_path: release_notes.md
        draft: false
        prerelease: false

  build-release:
    name: Build Release
    needs: create-release
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            binary: claudeforge
          - os: macos-latest
            target: x86_64-apple-darwin
            binary: claudeforge
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            binary: claudeforge.exe
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
      with:
        target: ${{ matrix.target }}
    
    - name: Build release binary
      run: cargo build --release --target ${{ matrix.target }}
    
    - name: Create archive
      run: |
        mkdir -p release
        cp target/${{ matrix.target }}/release/${{ matrix.binary }} release/
        cp README.md LICENSE release/
        tar -czf claudeforge-${{ matrix.target }}.tar.gz -C release .
    
    - name: Upload Release Asset
      uses: actions/upload-release-asset@v1
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      with:
        upload_url: ${{ needs.create-release.outputs.upload_url }}
        asset_path: ./claudeforge-${{ matrix.target }}.tar.gz
        asset_name: claudeforge-${{ matrix.target }}.tar.gz
        asset_content_type: application/gzip
```

### Security Workflow (`security.yml`)
```yaml
name: Security

on:
  schedule:
    - cron: '0 2 * * 1'  # Weekly on Monday
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  audit:
    name: Security Audit
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
    
    - name: Install cargo-audit
      run: cargo install cargo-audit
    
    - name: Run security audit
      run: cargo audit
    
    - name: Run cargo deny
      run: |
        cargo install cargo-deny
        cargo deny check
```

## Testing Strategy

### CI Testing
- **Unit Tests**: Run all unit tests with `cargo test`
- **Integration Tests**: Execute integration test suite
- **Property Tests**: Run property-based tests if present
- **Multi-platform**: Test on Linux, macOS, and Windows
- **Multiple Rust Versions**: Test on stable and beta

### Release Testing
- **Build Verification**: Ensure binaries build successfully
- **Smoke Tests**: Basic functionality tests on built binaries
- **Archive Integrity**: Verify release archives are properly formed
- **Checksum Validation**: Generate and verify checksums

### Security Testing
- **Dependency Auditing**: Check for known vulnerabilities
- **License Compliance**: Verify license compatibility
- **Supply Chain**: Validate dependency sources

## Edge Cases & Error Handling

### CI Failures
- **Flaky Tests**: Implement retry mechanisms for unstable tests
- **Build Failures**: Clear error reporting and artifact preservation
- **Network Issues**: Robust caching and retry strategies
- **Platform Differences**: Handle platform-specific test variations

### Release Failures
- **Build Errors**: Fail fast with clear error messages
- **Upload Failures**: Implement retry mechanisms for asset uploads
- **Version Conflicts**: Detect and handle version mismatches
- **Incomplete Releases**: Rollback mechanisms for partial failures

### Security Issues
- **Vulnerability Detection**: Automated alerts and PR creation
- **License Violations**: Block builds with incompatible licenses
- **Supply Chain Attacks**: Verification of dependency integrity

## Dependencies

### GitHub Actions
- `actions/checkout@v4`: Repository checkout
- `dtolnay/rust-toolchain@stable`: Rust toolchain setup
- `actions/cache@v4`: Dependency caching
- `actions/create-release@v1`: Release creation
- `actions/upload-release-asset@v1`: Asset uploads

### Rust Tools
- `cargo-audit`: Security auditing
- `cargo-deny`: License and dependency checking
- `cargo-tarpaulin`: Code coverage (optional)
- `cargo-release`: Release management (optional)

### External Services
- GitHub API: Release management
- Dependabot: Automated dependency updates
- Security advisories: Vulnerability notifications

## Configuration

### Environment Variables
```yaml
env:
  CARGO_TERM_COLOR: always
  CARGO_INCREMENTAL: 0
  RUSTFLAGS: "-Dwarnings"
  RUST_BACKTRACE: 1
```

### Secrets Required
- `GITHUB_TOKEN`: Automatic releases and API access
- `CARGO_REGISTRY_TOKEN`: Crates.io publishing (optional)
- `GPG_PRIVATE_KEY`: Binary signing (optional)

### Dependabot Configuration
```yaml
# .github/dependabot.yml
version: 2
updates:
  - package-ecosystem: "cargo"
    directory: "/"
    schedule:
      interval: "weekly"
    open-pull-requests-limit: 10
```

## Documentation

### Workflow Documentation
- README updates with CI/CD status badges
- Contributing guidelines for PR requirements
- Release process documentation
- Security policy and vulnerability reporting

### Status Badges
```markdown
[![CI](https://github.com/iepathos/claudeforge/workflows/CI/badge.svg)](https://github.com/iepathos/claudeforge/actions)
[![Release](https://github.com/iepathos/claudeforge/workflows/Release/badge.svg)](https://github.com/iepathos/claudeforge/releases)
[![Security](https://github.com/iepathos/claudeforge/workflows/Security/badge.svg)](https://github.com/iepathos/claudeforge/actions)
```

### Integration with Justfile
Update Justfile to include local CI simulation:
```just
# Run CI checks locally
ci-local: fmt-check lint test doc-check
    cargo audit
    @echo "Local CI simulation completed!"
```

## Implementation Steps

1. **Create `.github/workflows/` directory structure**
2. **Implement CI workflow with basic testing**
3. **Add code quality checks (formatting, linting)**
4. **Implement security scanning workflow**
5. **Create release workflow with cross-platform builds**
6. **Add caching for improved performance**
7. **Configure Dependabot for automated updates**
8. **Add documentation and status badges**
9. **Test workflows with sample releases**
10. **Optimize and refine based on initial runs**

## Security Considerations

- **Token Management**: Use least-privilege access tokens
- **Dependency Scanning**: Regular automated security audits
- **Supply Chain Security**: Verify action and dependency integrity
- **Secret Handling**: Proper secret management and rotation
- **Branch Protection**: Enforce CI requirements before merging

## Performance Optimizations

- **Caching Strategy**: Aggressive caching of Rust dependencies
- **Parallel Jobs**: Maximize concurrent job execution
- **Incremental Builds**: Use incremental compilation where possible
- **Target Optimization**: Build only necessary targets
- **Artifact Management**: Efficient artifact storage and retrieval