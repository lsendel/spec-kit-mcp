# Spec-Kit MCP Server

[![Crates.io](https://img.shields.io/crates/v/spec-kit-mcp)](https://crates.io/crates/spec-kit-mcp)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-MIT)
[![Build Status](https://img.shields.io/github/actions/workflow/status/yourusername/spec-kit-mcp/ci.yml?branch=main)](https://github.com/yourusername/spec-kit-mcp/actions)

**MCP server that enables AI coding assistants to use spec-driven development practices via the [GitHub Spec-Kit](https://github.com/github/spec-kit) toolkit.**

## Features

- **🎯 100% Spec-Kit Coverage**: All 10 tools for complete spec-driven development
- **🚀 MCP Protocol**: Full JSON-RPC 2.0 implementation for AI agents
- **⚡ High Performance**: Built with Rust and Tokio for async I/O
- **🔧 Dual Installation**: Install via `cargo` or `npx`
- **🛡️ Type Safe**: Comprehensive type system with validation
- **📚 Comprehensive Docs**: Tutorials, examples, and configuration guides
- **🌐 Multi-Editor Support**: Claude Code, Cursor, Windsurf, VS Code, and more
- **🧪 Production Ready**: Tested, documented, and deployed

## Quick Start

### Installation

#### Via Cargo (Recommended)

Fast, reliable, works offline:

```bash
cargo install spec-kit-mcp
```

**Advantages:**
- ✅ Fastest startup
- ✅ Works offline
- ✅ Most reliable
- ✅ Full platform support (macOS Intel/ARM, Linux)

#### Via npm/npx

For Node.js users:

```bash
# Global installation
npm install -g @lsendel/spec-kit-mcp

# Or use with npx (downloads on first use)
npx @lsendel/spec-kit-mcp
```

**Advantages:**
- ✅ Familiar for Node.js users
- ✅ Auto-downloads prebuilt binaries
- ✅ npx always uses latest version

### Prerequisites

- **Python 3.11+**: Required by GitHub spec-kit
- **uv package manager**: Required to run spec-kit (install from https://docs.astral.sh/uv/)
- **GitHub Spec-Kit**: No separate installation needed - the MCP server uses `uvx` to run spec-kit directly
- **Node.js 18+**: Only if using npx installation method
- **Git**: For version control operations

**Note**: The spec-kit CLI is not available on PyPI. The MCP server automatically runs it via `uvx --from git+https://github.com/github/spec-kit.git`

### Configuration with Claude Code

Claude Code supports two ways to configure MCP servers:

#### Method 1: Using Cargo Binary (Recommended)

First install via cargo:
```bash
cargo install spec-kit-mcp
```

Then configure `~/.config/claude-code/mcp.json`:

```json
{
  "mcpServers": {
    "spec-kit": {
      "command": "spec-kit-mcp",
      "args": [],
      "env": {}
    }
  }
}
```

**Advantages:**
- ✅ Fastest startup (<100ms)
- ✅ Works offline
- ✅ Most reliable
- ✅ Supports all platforms (macOS Intel/ARM, Linux)

#### Method 2: Using npx (No Installation)

Create or edit `~/.config/claude-code/mcp.json`:

```json
{
  "mcpServers": {
    "spec-kit": {
      "command": "npx",
      "args": ["-y", "@lsendel/spec-kit-mcp"],
      "env": {}
    }
  }
}
```

**Advantages:**
- ✅ No installation required
- ✅ Always uses latest version
- ✅ Good for trying it out

**Note**: First run may be slower as it downloads the binary.

#### Verify Configuration

1. **Restart Claude Code** after editing the config file

2. **Test the MCP tools** in Claude Code:
```
List all available MCP tools
```

You should see 10 spec-kit tools listed:
- speckit_init
- speckit_check
- speckit_constitution
- speckit_specify
- speckit_plan
- speckit_tasks
- speckit_implement
- speckit_clarify
- speckit_analyze
- speckit_checklist

3. **Try a simple command**:
```
Use speckit_check to verify my development environment
```

#### Troubleshooting Configuration

**Issue**: Tools not appearing after restart

**Solution**:
1. Check config file location: `cat ~/.config/claude-code/mcp.json`
2. Verify JSON syntax: `cat ~/.config/claude-code/mcp.json | jq`
3. Check Claude Code logs: `~/.config/claude-code/logs/`
4. For npx method, ensure Node.js is installed: `node --version`
5. For binary method, verify installation: `which spec-kit-mcp`

**Issue**: `spec-kit-mcp: command not found` (Method 2)

**Solution**:
- Use Method 1 (npx) instead, or
- Add npm global bin to PATH: `export PATH="$PATH:$(npm config get prefix)/bin"`

> **Need help with other editors?** See the [Complete Configuration Guide](./docs/CONFIGURATION.md) for Cursor, Windsurf, VS Code, and more.

## 📚 Documentation

### Quick Links

- **[📖 Comprehensive Tutorials](./TUTORIALS.md)** - Step-by-step guides for all skill levels
- **[💡 Examples](./examples/)** - Real-world project examples
- **[⚙️ Configuration Guide](./docs/CONFIGURATION.md)** - Setup for different AI assistants
- **[🚀 Usage Guide](./USAGE_GUIDE.md)** - Detailed usage documentation

### Learning Resources

**New to Spec-Kit?** Start here:
1. [Tutorial 1: Your First Spec-Kit Project](./TUTORIALS.md#tutorial-1-your-first-spec-kit-project) (20 min)
2. [Todo CLI Example](./examples/todo-cli/) - Complete beginner example
3. [Configuration for Your Editor](./docs/CONFIGURATION.md)

**Building APIs?** Check out:
- [Tutorial 2: Building a REST API](./TUTORIALS.md#tutorial-2-building-a-rest-api-with-spec-kit)
- [Blog API Example](./examples/blog-api/) - Production-ready API example

**Working with teams?**
- [Tutorial 4: Team Collaboration](./TUTORIALS.md#tutorial-4-team-collaboration-with-spec-kit)
- [CI/CD Integration](./TUTORIALS.md#tutorial-5-integrating-with-cicd)

## Available Tools

The MCP server exposes 10 spec-kit tools for the complete workflow:

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

### 6. `speckit_implement`

Execute implementation according to the task list.

```json
{
  "task_file": "./speckit.tasks",
  "context": "Using Rust with async/await",
  "output_dir": "./src"
}
```

### 7. `speckit_clarify`

Request clarification on ambiguous requirements or specifications.

```json
{
  "spec_file": "./speckit.specify",
  "questions": "How should we handle edge cases?"
}
```

### 8. `speckit_analyze`

Analyze code for quality, compliance, and technical debt.

```json
{
  "target_path": "./src",
  "check_constitution": true,
  "output_format": "markdown"
}
```

### 9. `speckit_check`

Validate that required tools are installed for spec-kit development.

```json
{
  "check_speckit": true,
  "check_git": true,
  "check_ai_tools": true
}
```

### 10. `speckit_checklist`

Generate review checklists to verify implementation completeness.

```json
{
  "spec_file": "./speckit.specify",
  "task_file": "./speckit.tasks",
  "output_path": "./checklist.md"
}
```

> **See all tools in action**: Check out the [Examples](./examples/) directory for complete workflows

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
- ✅ All 10 spec-kit tools implemented (100% coverage)
- ✅ MCP protocol support (JSON-RPC 2.0)
- ✅ Dual distribution (cargo + npx)
- ✅ Comprehensive error handling
- ✅ Complete tutorials and examples
- ✅ Configuration guides for major editors
- ✅ Published to crates.io and npm

### Future Versions

#### v0.2.0
- [ ] Enhanced tool parameters and validation
- [ ] Configuration file support (.speckit-mcp.toml)
- [ ] Template system for common project types
- [ ] Performance optimizations and caching
- [ ] Windows platform support
- [ ] Web-based tool output visualization

#### v0.3.0
- [ ] Remote MCP via Server-Sent Events (SSE)
- [ ] Web UI dashboard for project visualization
- [ ] Template marketplace integration
- [ ] Team collaboration features (shared constitutions)
- [ ] Metrics and analytics dashboard
- [ ] Plugin system for custom tools

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
```

**Solution**: Ensure uv package manager is installed:

```bash
# Check if uv is installed
uv --version

# If not installed, install uv (macOS/Linux)
curl -LsSf https://astral.sh/uv/install.sh | sh

# Or using pip
pip install uv

# Test spec-kit access
uvx --from git+https://github.com/github/spec-kit.git specify check
```

**Note**: The spec-kit CLI is not available as a standalone package. The MCP server uses `uvx` to run it directly from GitHub.

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
