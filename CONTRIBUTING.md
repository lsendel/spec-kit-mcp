# Contributing to Spec-Kit MCP

Thank you for your interest in contributing to spec-kit-mcp! This document provides guidelines for contributing to the project.

## Code of Conduct

Be respectful, inclusive, and considerate in all interactions.

## Getting Started

### Prerequisites

- Rust 1.70+ (2021 edition)
- cargo
- git
- Python 3.11+ (for testing with spec-kit)
- spec-kit CLI: `uv tool install specify-cli`

### Setup

```bash
# Fork and clone the repository
git clone https://github.com/yourusername/spec-kit-mcp.git
cd spec-kit-mcp

# Build the project
cargo build

# Run tests
cargo test

# Run the binary
cargo run -- --help
```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/my-feature
# or
git checkout -b fix/issue-123
```

### 2. Make Changes

- Write clean, idiomatic Rust code
- Follow existing code style
- Add tests for new functionality
- Update documentation as needed

### 3. Test Your Changes

```bash
# Run all tests
cargo test

# Run clippy
cargo clippy --all-targets --all-features

# Format code
cargo fmt

# Build release
cargo build --release
```

### 4. Commit Changes

Use [Conventional Commits](https://www.conventionalcommits.org/):

```bash
git commit -m "feat: add new tool for X"
git commit -m "fix: correct error handling in Y"
git commit -m "docs: update README with examples"
git commit -m "test: add tests for Z"
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `test`: Test changes
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `chore`: Build/tooling changes

### 5. Push and Create PR

```bash
git push origin feature/my-feature
```

Then create a pull request on GitHub.

## Project Structure

```
spec-kit-mcp/
├── src/
│   ├── main.rs              # Binary entry point
│   ├── lib.rs               # Library root
│   ├── mcp/                 # MCP protocol
│   ├── speckit/             # Spec-kit integration
│   ├── tools/               # Tool implementations
│   ├── config/              # Configuration
│   └── utils/               # Utilities
├── tests/                   # Integration tests
├── npm-package/             # NPM distribution
└── .github/workflows/       # CI/CD
```

## Adding a New Tool

To add a new spec-kit tool:

1. **Create tool file**: `src/tools/mytool.rs`

```rust
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::mcp::types::{ContentBlock, ToolDefinition, ToolResult};
use crate::speckit::SpecKitCli;
use crate::tools::Tool;

#[derive(Debug, Deserialize, Serialize)]
pub struct MyToolParams {
    // Parameters
}

pub struct MyTool {
    cli: SpecKitCli,
}

impl MyTool {
    pub fn new(cli: SpecKitCli) -> Self {
        Self { cli }
    }
}

#[async_trait]
impl Tool for MyTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "speckit_mytool".to_string(),
            description: "Description".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {},
                "required": []
            })
        }
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        // Implementation
        Ok(ToolResult {
            content: vec![ContentBlock::text("Result")],
            is_error: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mytool() {
        // Tests
    }
}
```

2. **Register in mod.rs**:

```rust
// In src/tools/mod.rs
pub mod mytool;
pub use mytool::MyTool;

// In create_registry function:
registry.register(Arc::new(MyTool::new(cli.clone())));
```

3. **Add tests**:

```rust
#[tokio::test]
async fn test_mytool_execution() {
    // Test implementation
}
```

4. **Update documentation**:
   - Add to README.md tools list
   - Add example usage
   - Update CHANGELOG.md

## Testing Guidelines

### Unit Tests

- Test each function independently
- Use `#[tokio::test]` for async tests
- Mock external dependencies when possible

```rust
#[tokio::test]
async fn test_function() {
    let result = my_function().await.unwrap();
    assert_eq!(result, expected);
}
```

### Integration Tests

- Place in `tests/` directory
- Test complete workflows
- Test error scenarios

```rust
// tests/integration_test.rs
#[tokio::test]
async fn test_complete_workflow() {
    // Test end-to-end functionality
}
```

### Test Coverage

Aim for >80% code coverage:

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage
cargo tarpaulin --out Html
```

## Documentation Guidelines

### Code Documentation

- Add rustdoc comments to all public items
- Include examples in doc comments
- Document parameters and return values

```rust
/// Creates a new specification file.
///
/// # Arguments
///
/// * `requirements` - The requirements to specify
/// * `output_path` - Where to write the file
///
/// # Examples
///
/// ```
/// # use spec_kit_mcp::*;
/// # tokio_test::block_on(async {
/// let result = specify("requirements", "./spec").await?;
/// # Ok::<(), anyhow::Error>(())
/// # });
/// ```
///
/// # Errors
///
/// Returns error if file cannot be written
pub async fn specify(requirements: &str, output_path: &Path) -> Result<()> {
    // Implementation
}
```

### Generate Documentation

```bash
cargo doc --no-deps --open
```

## Performance Guidelines

- Use async/await for I/O operations
- Avoid unnecessary allocations
- Profile before optimizing

```bash
# Benchmark
cargo bench

# Profile
cargo flamegraph
```

## Pull Request Process

1. **Ensure CI passes**
   - All tests pass
   - Clippy has no warnings
   - Code is formatted

2. **Update documentation**
   - README if adding features
   - CHANGELOG.md with changes
   - Rust docs for new APIs

3. **Request review**
   - Tag maintainers
   - Address feedback
   - Update as needed

4. **Merge**
   - Squash commits if needed
   - Merge when approved

## Release Process

Maintainers follow this process:

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Commit: `git commit -m "chore: release v0.X.0"`
4. Tag: `git tag v0.X.0`
5. Push: `git push origin main --tags`
6. GitHub Actions handles the rest

## Questions?

- Open an issue for bugs/features
- Start a discussion for questions
- Join our community chat (if available)

## License

By contributing, you agree that your contributions will be licensed under the same dual MIT/Apache-2.0 license as the project.

---

Thank you for contributing! 🚀
