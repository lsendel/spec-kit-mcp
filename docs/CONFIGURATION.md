# Configuration Guide

Complete configuration guide for using Spec-Kit MCP with different AI coding assistants.

## Table of Contents

- [Claude Code Configuration](#claude-code-configuration)
- [Cursor Configuration](#cursor-configuration)
- [Windsurf Configuration](#windsurf-configuration)
- [VS Code with MCP](#vs-code-with-mcp)
- [Zed Editor Configuration](#zed-editor-configuration)
- [Custom MCP Client](#custom-mcp-client)

---

## Claude Code Configuration

Claude Code has built-in MCP support and automatically detects MCP servers.

### Setup Steps

1. **Install spec-kit-mcp**:

```bash
# Via npm (recommended for Claude Code)
npm install -g @lsendel/spec-kit-mcp

# Or via cargo
cargo install spec-kit-mcp
```

2. **Verify Installation**:

```bash
spec-kit-mcp --version
```

3. **Configure MCP**:

Create or update `~/.config/claude-code/mcp.json`:

```json
{
  "mcpServers": {
    "spec-kit": {
      "command": "spec-kit-mcp",
      "args": [],
      "env": {
        "SPEC_KIT_LOG_LEVEL": "info"
      }
    }
  }
}
```

**Or with npx** (no installation needed):

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

4. **Restart Claude Code**:

```bash
# Close and reopen Claude Code, or use the command palette
# Command + Shift + P (Mac) or Ctrl + Shift + P (Windows/Linux)
# Then: "Reload Window"
```

5. **Verify Tools are Available**:

In Claude Code, type:
```
List all available MCP tools
```

You should see 10 spec-kit tools:
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

### Usage with Claude Code

**Example prompts**:

```
Use speckit_init to create a new project named "my-app"
```

```
Use speckit_specify to create a specification for user authentication
```

```
Use speckit_check to verify my development environment
```

### Troubleshooting

**Issue**: Tools not appearing

**Solution**:
1. Check MCP configuration file exists: `cat ~/.config/claude-code/mcp.json`
2. Verify command is accessible: `which spec-kit-mcp`
3. Check Claude Code logs: `~/.config/claude-code/logs/`
4. Restart Claude Code

**Issue**: `spec-kit-mcp: command not found`

**Solution**:
- Use npx version in config
- Or add npm global bin to PATH: `export PATH="$PATH:$(npm config get prefix)/bin"`

---

## Cursor Configuration

Cursor supports MCP through configuration files.

### Setup Steps

1. **Install spec-kit-mcp**:

```bash
npm install -g @lsendel/spec-kit-mcp
```

2. **Configure MCP**:

Create or update `~/.cursor/mcp.json`:

```json
{
  "mcpServers": {
    "spec-kit": {
      "command": "spec-kit-mcp",
      "args": [],
      "env": {},
      "disabled": false
    }
  }
}
```

**Using npx**:

```json
{
  "mcpServers": {
    "spec-kit": {
      "command": "npx",
      "args": ["-y", "@lsendel/spec-kit-mcp"],
      "env": {},
      "disabled": false
    }
  }
}
```

3. **Restart Cursor**:

Close and reopen Cursor.

4. **Verify Installation**:

Open Cursor's AI chat and type:
```
Show me available MCP servers and their tools
```

### Cursor-Specific Features

**Composer Mode**: Use spec-kit tools in Cursor's Composer:

```
@spec-kit Create a specification for a REST API endpoint
```

**Inline Chat**: Use spec-kit for inline code generation:

```
cmd+k (Mac) or ctrl+k (Windows/Linux)
Then: "Use speckit_implement to add this feature"
```

### Configuration Options

**Custom working directory**:

```json
{
  "mcpServers": {
    "spec-kit": {
      "command": "spec-kit-mcp",
      "args": [],
      "env": {
        "SPEC_KIT_WORK_DIR": "${workspaceFolder}"
      },
      "cwd": "${workspaceFolder}"
    }
  }
}
```

**Enable debug logging**:

```json
{
  "mcpServers": {
    "spec-kit": {
      "command": "spec-kit-mcp",
      "args": [],
      "env": {
        "SPEC_KIT_LOG_LEVEL": "debug",
        "RUST_LOG": "debug"
      }
    }
  }
}
```

### Troubleshooting

**Issue**: MCP server not starting

**Solution**:
1. Check configuration syntax: `cat ~/.cursor/mcp.json | jq`
2. Test command manually: `spec-kit-mcp`
3. Check Cursor logs: Help → Show Logs

---

## Windsurf Configuration

Windsurf (Codeium's IDE) supports MCP through its configuration.

### Setup Steps

1. **Install spec-kit-mcp**:

```bash
npm install -g @lsendel/spec-kit-mcp
```

2. **Configure MCP**:

Create or update `~/.windsurf/mcp-config.json`:

```json
{
  "servers": {
    "spec-kit": {
      "command": "spec-kit-mcp",
      "args": [],
      "env": {},
      "autoStart": true
    }
  }
}
```

**Or with npx**:

```json
{
  "servers": {
    "spec-kit": {
      "command": "npx",
      "args": ["-y", "@lsendel/spec-kit-mcp"],
      "env": {},
      "autoStart": true
    }
  }
}
```

3. **Restart Windsurf**:

Close and reopen the application.

4. **Verify**:

Use Windsurf's Cascade feature:
```
Check what MCP tools are available
```

### Using with Cascade

Windsurf's Cascade feature works great with spec-kit:

**Flow mode**:
```
Use spec-kit to create specifications for this project, then implement them step by step
```

**Supercomplete**:
```
# Start typing a specification
# Cascade will use spec-kit tools to complete it
```

### Configuration Options

**Per-project configuration**:

Create `.windsurf/mcp.json` in your project root:

```json
{
  "servers": {
    "spec-kit": {
      "command": "npx",
      "args": ["-y", "@lsendel/spec-kit-mcp"],
      "env": {
        "SPEC_KIT_PROJECT_ROOT": "${workspaceFolder}"
      }
    }
  }
}
```

---

## VS Code with MCP

VS Code supports MCP through extensions.

### Setup Steps

1. **Install MCP Extension**:

Search for "Model Context Protocol" in VS Code extensions.

2. **Install spec-kit-mcp**:

```bash
npm install -g @lsendel/spec-kit-mcp
```

3. **Configure in VS Code Settings**:

Open Settings (JSON) and add:

```json
{
  "mcp.servers": {
    "spec-kit": {
      "command": "spec-kit-mcp",
      "args": [],
      "env": {}
    }
  }
}
```

4. **Use with GitHub Copilot Chat**:

In Copilot Chat:
```
@spec-kit Use speckit_specify to create a new feature spec
```

### Workspace Configuration

Create `.vscode/settings.json` in your project:

```json
{
  "mcp.servers": {
    "spec-kit": {
      "command": "npx",
      "args": ["-y", "@lsendel/spec-kit-mcp"],
      "env": {
        "SPEC_KIT_WORK_DIR": "${workspaceFolder}"
      }
    }
  },
  "files.watcherExclude": {
    "**/.speckit/cache/**": true
  }
}
```

---

## Zed Editor Configuration

Zed has experimental MCP support.

### Setup Steps

1. **Enable MCP feature**:

In Zed settings (`~/.config/zed/settings.json`):

```json
{
  "features": {
    "mcp": true
  }
}
```

2. **Configure MCP servers**:

```json
{
  "mcp": {
    "servers": {
      "spec-kit": {
        "command": "npx",
        "args": ["-y", "@lsendel/spec-kit-mcp"]
      }
    }
  }
}
```

3. **Restart Zed**

---

## Custom MCP Client

Using spec-kit-mcp with a custom MCP client.

### Direct JSON-RPC Communication

```typescript
import { spawn } from 'child_process';
import { createInterface } from 'readline';

class SpecKitMCPClient {
  private process: any;
  private requestId = 0;

  async start() {
    this.process = spawn('spec-kit-mcp', [], {
      stdio: ['pipe', 'pipe', 'pipe']
    });

    const readline = createInterface({
      input: this.process.stdout
    });

    readline.on('line', (line) => {
      const message = JSON.parse(line);
      this.handleMessage(message);
    });
  }

  async callTool(toolName: string, params: any) {
    const request = {
      jsonrpc: '2.0',
      id: ++this.requestId,
      method: 'tools/call',
      params: {
        name: toolName,
        arguments: params
      }
    };

    this.process.stdin.write(JSON.stringify(request) + '\n');
  }

  private handleMessage(message: any) {
    console.log('Received:', message);
  }
}

// Usage
const client = new SpecKitMCPClient();
await client.start();
await client.callTool('speckit_check', {
  check_speckit: true,
  check_git: true
});
```

### Python Client Example

```python
import subprocess
import json

class SpecKitMCPClient:
    def __init__(self):
        self.process = subprocess.Popen(
            ['spec-kit-mcp'],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        self.request_id = 0

    def call_tool(self, tool_name: str, params: dict):
        self.request_id += 1
        request = {
            'jsonrpc': '2.0',
            'id': self.request_id,
            'method': 'tools/call',
            'params': {
                'name': tool_name,
                'arguments': params
            }
        }

        self.process.stdin.write(json.dumps(request) + '\n')
        self.process.stdin.flush()

        response = self.process.stdout.readline()
        return json.loads(response)

# Usage
client = SpecKitMCPClient()
result = client.call_tool('speckit_check', {
    'check_speckit': True,
    'check_git': True
})
print(result)
```

---

## Environment Variables

All configurations support these environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `SPEC_KIT_LOG_LEVEL` | Logging level (trace, debug, info, warn, error) | `info` |
| `SPEC_KIT_WORK_DIR` | Working directory for spec-kit operations | Current directory |
| `SPEC_KIT_PROJECT_ROOT` | Project root directory | Auto-detected (git root) |
| `SPEC_KIT_CACHE_DIR` | Cache directory for temporary files | `.speckit/cache` |
| `RUST_LOG` | Rust logging configuration | `spec_kit_mcp=info` |
| `NO_COLOR` | Disable colored output | Not set |

### Example with environment variables:

```json
{
  "mcpServers": {
    "spec-kit": {
      "command": "spec-kit-mcp",
      "args": [],
      "env": {
        "SPEC_KIT_LOG_LEVEL": "debug",
        "SPEC_KIT_PROJECT_ROOT": "/path/to/project",
        "RUST_LOG": "spec_kit_mcp=trace"
      }
    }
  }
}
```

---

## Advanced Configuration

### Multiple Project Support

Configure different spec-kit instances for different project types:

```json
{
  "mcpServers": {
    "spec-kit-rust": {
      "command": "spec-kit-mcp",
      "env": {
        "SPEC_KIT_PROJECT_TYPE": "rust"
      }
    },
    "spec-kit-node": {
      "command": "spec-kit-mcp",
      "env": {
        "SPEC_KIT_PROJECT_TYPE": "node"
      }
    }
  }
}
```

### Custom Tool Paths

If you need to specify custom paths for tools (git, specify, etc.):

```json
{
  "mcpServers": {
    "spec-kit": {
      "command": "spec-kit-mcp",
      "env": {
        "GIT_PATH": "/custom/path/to/git",
        "SPECIFY_PATH": "/custom/path/to/specify"
      }
    }
  }
}
```

### Performance Tuning

For large projects, optimize performance:

```json
{
  "mcpServers": {
    "spec-kit": {
      "command": "spec-kit-mcp",
      "env": {
        "SPEC_KIT_CACHE_ENABLED": "true",
        "SPEC_KIT_CACHE_SIZE": "100MB",
        "SPEC_KIT_PARALLEL_TASKS": "4"
      }
    }
  }
}
```

---

## Verification

### Test Your Configuration

Create a test script to verify your configuration:

```bash
#!/bin/bash
# test-spec-kit-config.sh

echo "Testing spec-kit-mcp configuration..."

# Test 1: Check if command is available
echo "1. Checking if spec-kit-mcp is installed..."
if command -v spec-kit-mcp &> /dev/null; then
    echo "   ✅ spec-kit-mcp is installed"
    spec-kit-mcp --version
else
    echo "   ❌ spec-kit-mcp not found"
    exit 1
fi

# Test 2: Verify MCP configuration
echo "2. Checking MCP configuration..."
CONFIG_FILE="$HOME/.config/claude-code/mcp.json"  # Adjust for your editor
if [ -f "$CONFIG_FILE" ]; then
    echo "   ✅ MCP config found at $CONFIG_FILE"
    if jq empty "$CONFIG_FILE" 2>/dev/null; then
        echo "   ✅ MCP config is valid JSON"
    else
        echo "   ❌ MCP config is invalid JSON"
        exit 1
    fi
else
    echo "   ⚠️  MCP config not found at $CONFIG_FILE"
fi

# Test 3: Test MCP server startup
echo "3. Testing MCP server startup..."
timeout 5s spec-kit-mcp 2>&1 | grep -q "MCP server" && \
    echo "   ✅ MCP server starts successfully" || \
    echo "   ❌ MCP server failed to start"

echo ""
echo "Configuration test complete!"
```

Run with:
```bash
chmod +x test-spec-kit-config.sh
./test-spec-kit-config.sh
```

---

## Troubleshooting Guide

### Common Issues

#### 1. "command not found: spec-kit-mcp"

**Cause**: spec-kit-mcp not in PATH

**Solutions**:
- Use npx version in config: `"command": "npx", "args": ["-y", "@lsendel/spec-kit-mcp"]`
- Add to PATH: `export PATH="$PATH:$(npm config get prefix)/bin"`
- Use full path: `"command": "/full/path/to/spec-kit-mcp"`

#### 2. MCP Tools Not Appearing

**Cause**: MCP server not properly configured or not starting

**Solutions**:
1. Verify configuration file syntax
2. Check editor logs
3. Restart editor
4. Test server manually: `spec-kit-mcp`

#### 3. Permission Denied

**Cause**: Binary not executable

**Solution**:
```bash
chmod +x $(which spec-kit-mcp)
```

#### 4. Tools Return Errors

**Cause**: Working directory issues or missing dependencies

**Solutions**:
- Ensure git is installed
- Check project is initialized
- Verify working directory is correct

### Debug Mode

Enable debug logging to troubleshoot issues:

```json
{
  "mcpServers": {
    "spec-kit": {
      "command": "spec-kit-mcp",
      "env": {
        "SPEC_KIT_LOG_LEVEL": "trace",
        "RUST_LOG": "spec_kit_mcp=trace",
        "RUST_BACKTRACE": "1"
      }
    }
  }
}
```

Check logs in:
- Claude Code: `~/.config/claude-code/logs/`
- Cursor: `~/.cursor/logs/`
- Windsurf: `~/.windsurf/logs/`

---

## Getting Help

- **Documentation**: [GitHub](https://github.com/lsendel/spec-kit-mcp)
- **Issues**: [GitHub Issues](https://github.com/lsendel/spec-kit-mcp/issues)
- **Discussions**: [GitHub Discussions](https://github.com/lsendel/spec-kit-mcp/discussions)

---

**Ready to start?** Check out the [Examples](../examples/README.md) and [Tutorials](../TUTORIALS.md)!
