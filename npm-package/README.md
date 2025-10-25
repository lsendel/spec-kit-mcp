# @speckit/mcp

**MCP server for GitHub Spec-Kit - enables AI coding assistants to use spec-driven development**

This is the NPM distribution of the [spec-kit-mcp](https://github.com/yourusername/spec-kit-mcp) server, built with Rust for high performance.

## Quick Start

### Using with Claude Code

Add to your `.mcp.json`:

```json
{
  "mcpServers": {
    "spec-kit": {
      "type": "stdio",
      "command": "npx",
      "args": ["@speckit/mcp@latest"],
      "env": {}
    }
  }
}
```

### Using with Cursor

Add to your Cursor settings:

```json
{
  "mcp": {
    "servers": {
      "spec-kit": {
        "command": "npx",
        "args": ["@speckit/mcp@latest"]
      }
    }
  }
}
```

## Prerequisites

- **Node.js 18+**: Required for npx
- **Spec-Kit CLI**: Install with `uv tool install specify-cli`
- **Python 3.11+**: Required by spec-kit

## Installation

### Via NPX (Recommended)

No installation needed! The binary is downloaded automatically on first use.

```bash
npx @speckit/mcp --help
```

### Via NPM (Global)

```bash
npm install -g @speckit/mcp
spec-kit-mcp --help
```

### Via NPM (Local)

```bash
npm install @speckit/mcp
npx spec-kit-mcp --help
```

## Available Tools

1. **speckit_init** - Initialize a new spec-kit project
2. **speckit_constitution** - Create governing principles
3. **speckit_specify** - Define requirements and user stories
4. **speckit_plan** - Create technical implementation plan
5. **speckit_tasks** - Generate actionable task list

## Usage Example

```
User: Initialize a spec-kit project for my new feature

Claude: [Uses speckit_init tool]
✓ Project initialized

User: Create a constitution focusing on security and simplicity

Claude: [Uses speckit_constitution tool]
✓ Constitution created

User: Specify requirements for OAuth2 authentication

Claude: [Uses speckit_specify tool]
✓ Requirements specified

User: Create a technical plan

Claude: [Uses speckit_plan tool]
✓ Plan created

User: Generate tasks

Claude: [Uses speckit_tasks tool]
✓ 23 tasks generated
```

## Platform Support

- ✅ macOS (Intel)
- ✅ macOS (Apple Silicon)
- ✅ Linux (x86_64)
- ⏳ Windows (coming soon)

## Troubleshooting

### Binary Download Fails

If automatic download fails, you can build from source:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install spec-kit-mcp
cargo install spec-kit-mcp

# Use the cargo-installed version in .mcp.json
{
  "command": "spec-kit-mcp",
  "args": []
}
```

### Spec-Kit CLI Not Found

```bash
# Install spec-kit CLI
uv tool install specify-cli

# Or with pip
pip install specify-cli
```

### Permission Denied (macOS/Linux)

```bash
chmod +x $(which spec-kit-mcp)
```

## Documentation

- **Full Documentation**: https://github.com/yourusername/spec-kit-mcp
- **Spec-Kit**: https://github.com/github/spec-kit
- **MCP Protocol**: https://modelcontextprotocol.io/

## Performance

- **Cold start**: <500ms
- **Tool invocation**: <200ms
- **Memory usage**: <50MB

## License

Dual-licensed under MIT or Apache-2.0, at your option.

## Links

- **GitHub**: https://github.com/yourusername/spec-kit-mcp
- **Issues**: https://github.com/yourusername/spec-kit-mcp/issues
- **Rust Crate**: https://crates.io/crates/spec-kit-mcp
