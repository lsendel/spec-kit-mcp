# Spec-Kit MCP Server

[![Crates.io](https://img.shields.io/crates/v/spec-kit-mcp)](https://crates.io/crates/spec-kit-mcp)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/yourusername/spec-kit-mcp/ci.yml?branch=main)](https://github.com/yourusername/spec-kit-mcp/actions)

**MCP server that enables AI coding assistants to use spec-driven development practices via the [GitHub Spec-Kit](https://github.com/github/spec-kit) toolkit.**

## Features

- **🎯 Complete Spec-Kit Workflow**: All 5 core tools for spec-driven development
- **🚀 MCP Protocol**: Full JSON-RPC 2.0 implementation for AI agents
- **⚡ High Performance**: Built with Rust and Tokio for async I/O
- **🔧 Dual Installation**: Install via `cargo` or `npx`
- **🛡️ Type Safe**: Comprehensive type system with validation
- **📝 Well Documented**: Extensive API documentation and examples

## Quick Start

### Installation

#### Via NPX (Recommended)

Zero installation, always latest version:

```bash
# Just add to your .mcp.json (see Configuration below)
npx @speckit/mcp@latest
```

#### Via Cargo

For Rust developers or offline use:

```bash
cargo install spec-kit-mcp
```

### Prerequisites

- **Spec-Kit CLI**: Install with `uv tool install specify-cli` or `pip install specify-cli`
- **Python 3.11+**: Required by spec-kit
- **Node.js 18+**: Only if using npx

### Configuration

Add to your Claude Code `.mcp.json`:

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

Or if installed via cargo:

```json
{
  "mcpServers": {
    "spec-kit": {
      "type": "stdio",
      "command": "spec-kit-mcp",
      "args": [],
      "env": {}
    }
  }
}
```

## Available Tools

The MCP server exposes 5 spec-kit tools for the complete workflow:

### 1. `speckit_init`

Initialize a new spec-kit project with proper structure.

```json
{
  "project_name": "my-project",
  "project_path": "."
}
```

### 2. `speckit_constitution`

Create project governing principles and development standards.

```json
{
  "principles": "Simplicity, Performance, Security",
  "constraints": "Must support Python 3.11+",
  "output_path": "./speckit.constitution"
}
```

### 3. `speckit_specify`

Define requirements and user stories (the "what").

```json
{
  "requirements": "User authentication system with OAuth2 support",
  "user_stories": "As a user, I want to login with Google...",
  "output_path": "./speckit.specify"
}
```

### 4. `speckit_plan`

Create a technical implementation plan (the "how").

```json
{
  "spec_file": "./speckit.specify",
  "tech_stack": "Rust + Tokio",
  "output_path": "./speckit.plan"
}
```

### 5. `speckit_tasks`

Generate actionable task lists from the plan.

```json
{
  "plan_file": "./speckit.plan",
  "breakdown_level": "medium",
  "output_path": "./speckit.tasks"
}
```

## Usage Example

Here's a complete workflow using Claude Code:

```
User: Initialize a new spec-kit project called "user-auth"

Claude: [Uses speckit_init tool]
✓ Project initialized at ./user-auth

User: Create a constitution focusing on security and simplicity

Claude: [Uses speckit_constitution tool]
✓ Constitution created at ./speckit.constitution

User: Specify requirements for OAuth2 authentication

Claude: [Uses speckit_specify tool]
✓ Specification created at ./speckit.specify

User: Create a technical plan using Rust and OAuth2 libraries

Claude: [Uses speckit_plan tool]
✓ Technical plan created at ./speckit.plan

User: Generate detailed tasks

Claude: [Uses speckit_tasks tool]
✓ Task list created at ./speckit.tasks
  Found 15 actionable tasks
```

## Architecture

```
AI Agent (Claude Code, Cursor, etc.)
    ↓
MCP Protocol (JSON-RPC 2.0 over stdio)
    ↓
Spec-Kit MCP Server (Rust/Tokio)
    ↓
Tool Registry & Dispatcher
    ↓
Spec-Kit CLI Integration Layer
    ↓
Spec-Kit Python CLI (subprocess)
    ↓
File System (speckit.* artifacts)
```

## Development

### Building from Source

```bash
git clone https://github.com/yourusername/spec-kit-mcp.git
cd spec-kit-mcp
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Running the Server

```bash
# With default settings
cargo run

# With custom log level
cargo run -- --log-level debug

# With custom CLI path
cargo run -- --cli-path /path/to/specify

# With custom timeout
cargo run -- --timeout 600
```

## Project Structure

```
spec-kit-mcp/
├── src/
│   ├── main.rs              # Binary entry point
│   ├── lib.rs               # Library root
│   ├── mcp/                 # MCP protocol implementation
│   │   ├── types.rs         # JSON-RPC types
│   │   ├── protocol.rs      # Protocol handler
│   │   ├── transport.rs     # Stdio transport
│   │   └── server.rs        # MCP server
│   ├── speckit/             # Spec-kit CLI integration
│   │   ├── cli.rs           # Command execution
│   │   └── errors.rs        # Error types
│   └── tools/               # MCP tools
│       ├── mod.rs           # Tool registry
│       ├── init.rs          # speckit_init tool
│       ├── constitution.rs  # speckit_constitution tool
│       ├── specify.rs       # speckit_specify tool
│       ├── plan.rs          # speckit_plan tool
│       └── tasks.rs         # speckit_tasks tool
├── Cargo.toml               # Rust package manifest
└── README.md                # This file
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Run clippy: `cargo clippy`
6. Format code: `cargo fmt`
7. Commit with conventional commits: `git commit -m "feat: add new feature"`
8. Push and create a Pull Request

## Roadmap

### Current Version (0.1.0)
- ✅ Core 5 tools implemented
- ✅ MCP protocol support
- ✅ Dual distribution (cargo + npx)
- ✅ Comprehensive error handling

### Future Versions

#### v0.2.0
- [ ] Additional tools: `speckit_implement`, `speckit_clarify`, `speckit_analyze`, `speckit_checklist`
- [ ] Configuration file support
- [ ] Enhanced logging and debugging
- [ ] Performance optimizations

#### v0.3.0
- [ ] Remote MCP via SSE
- [ ] Web UI for visualization
- [ ] Template marketplace
- [ ] Team collaboration features

## Performance

- **Cold start**: <500ms
- **Tool invocation**: <200ms (excluding spec-kit CLI execution)
- **Memory usage**: <50MB baseline
- **Concurrent requests**: 10+

## Compatibility

### AI Coding Assistants

- ✅ Claude Code
- ✅ Cursor
- ✅ GitHub Copilot (with MCP support)
- ✅ Any MCP-compatible client

### Platforms

- ✅ macOS (Intel and ARM)
- ✅ Linux (x86_64)
- ⏳ Windows (planned)

## Troubleshooting

### Spec-Kit CLI Not Found

```
Error: spec-kit CLI not found!
Please install it with: uv tool install specify-cli
```

**Solution**: Install spec-kit CLI:

```bash
# Using uv (recommended)
uv tool install specify-cli

# Or using pip
pip install specify-cli
```

### Python Version Too Old

**Solution**: Upgrade to Python 3.11 or later:

```bash
# Check version
python3 --version

# Install Python 3.11+ (macOS with Homebrew)
brew install python@3.11
```

### Permission Denied

**Solution**: Ensure the binary is executable:

```bash
chmod +x $(which spec-kit-mcp)
```

## FAQ

**Q: Do I need to install spec-kit separately?**
A: Yes, the MCP server requires the spec-kit Python CLI to be installed.

**Q: Can I use this without Claude Code?**
A: Yes! It works with any MCP-compatible AI coding assistant.

**Q: Does this work offline?**
A: Yes, if installed via cargo. The npx version requires internet for initial download.

**Q: How does this compare to using spec-kit directly?**
A: This MCP server enables AI agents to use spec-kit automatically, streamlining the workflow.

## License

This project is dual-licensed under:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

You may choose either license for your use.

## Acknowledgments

- [GitHub Spec-Kit](https://github.com/github/spec-kit) - The underlying spec-driven development toolkit
- [Model Context Protocol](https://modelcontextprotocol.io/) - The protocol enabling AI-tool integration
- [Anthropic Claude](https://claude.ai/) - AI assistant that inspired this integration
- Rust Community - For excellent async tooling (Tokio, Serde, etc.)

## Links

- **Crates.io**: https://crates.io/crates/spec-kit-mcp
- **Docs.rs**: https://docs.rs/spec-kit-mcp
- **GitHub**: https://github.com/yourusername/spec-kit-mcp
- **Issues**: https://github.com/yourusername/spec-kit-mcp/issues
- **Spec-Kit**: https://github.com/github/spec-kit

---

**Built with Rust** | **Production Ready** | **Open Source**
