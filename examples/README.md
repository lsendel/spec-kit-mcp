# Spec-Kit MCP Examples

Practical examples demonstrating spec-driven development workflows using the Spec-Kit MCP Server.

## Examples Overview

### 1. [Todo CLI](./todo-cli/) - Beginner
**What it demonstrates**: Basic spec-kit workflow from specification to implementation
- Simple command-line application
- CRUD operations
- Local storage
- Unit testing
- **Time**: 30 minutes
- **Best for**: Learning the fundamentals

### 2. [Blog API](./blog-api/) - Intermediate
**What it demonstrates**: Building a REST API with authentication
- RESTful API design
- JWT authentication
- Database integration
- API testing
- Documentation generation
- **Time**: 60 minutes
- **Best for**: Web backend development

### 3. [Web App](./web-app/) - Intermediate
**What it demonstrates**: Full-stack development with React and Node.js
- Frontend and backend specifications
- Component-driven development
- State management
- Integration testing
- **Time**: 90 minutes
- **Best for**: Full-stack projects

### 4. [Refactoring Legacy Code](./refactoring/) - Advanced
**What it demonstrates**: Systematic refactoring of existing codebase
- Legacy code analysis
- Incremental refactoring strategy
- Strangler fig pattern
- Migration planning
- **Time**: 120 minutes
- **Best for**: Modernizing existing projects

## Quick Start

Each example includes:
- 📋 **Specifications** - Complete feature specs in `.speckit/specs/`
- 📝 **Plans** - Implementation plans in `.speckit/plans/`
- ✅ **Tasks** - Task checklists in `.speckit/tasks/`
- 💻 **Implementation** - Complete working code
- 🧪 **Tests** - Comprehensive test suites
- 📚 **Documentation** - Usage guides and API docs

## Running the Examples

### Prerequisites

```bash
# Install spec-kit-mcp
npm install -g @lsendel/spec-kit-mcp
# or
cargo install spec-kit-mcp

# Install your preferred AI coding assistant
# - Claude Code: https://docs.claude.com/claude-code
# - Cursor: https://cursor.sh
# - Windsurf: https://codeium.com/windsurf
```

### Running an Example

```bash
# Navigate to an example
cd examples/todo-cli

# Follow the README instructions
cat README.md

# Open in your AI assistant
code .  # or cursor . or windsurf .
```

## Learning Path

**New to spec-kit?** Follow this path:

1. Start with [Todo CLI](./todo-cli/) - Learn the basics
2. Move to [Blog API](./blog-api/) - Understand API development
3. Try [Web App](./web-app/) - Full-stack development
4. Challenge yourself with [Refactoring](./refactoring/) - Advanced patterns

## Using Examples with AI Assistants

### With Claude Code

```bash
cd examples/todo-cli
code .
```

Then in Claude:
```
I'm in the todo-cli example. Use the spec-kit MCP tools to walk me through building this application step by step.
```

### With Cursor

```bash
cd examples/todo-cli
cursor .
```

Use Cursor's AI with spec-kit tools to implement features.

### With Windsurf

```bash
cd examples/todo-cli
windsurf .
```

Cascade through the implementation using spec-kit tools.

## Example Structure

Each example follows this structure:

```
example-name/
├── README.md                 # Example-specific guide
├── .speckit/
│   ├── CONSTITUTION.md       # Project principles
│   ├── specs/               # Feature specifications
│   │   ├── feature-1.md
│   │   └── feature-2.md
│   ├── plans/               # Implementation plans
│   │   ├── feature-1-plan.md
│   │   └── feature-2-plan.md
│   └── tasks/               # Task checklists
│       ├── feature-1-tasks.md
│       └── feature-2-tasks.md
├── src/                     # Source code
├── tests/                   # Test suites
└── docs/                    # Additional documentation
```

## Key Learning Points

### From Todo CLI

- How to initialize a spec-kit project
- Writing clear specifications
- Generating implementation plans
- Using task checklists effectively

### From Blog API

- API design with specifications
- Authentication and authorization
- Database integration strategies
- API testing approaches

### From Web App

- Full-stack coordination
- Frontend/backend specifications
- State management patterns
- End-to-end testing

### From Refactoring

- Analyzing legacy code
- Creating refactoring strategies
- Incremental migration
- Maintaining backward compatibility

## Contributing Your Own Examples

Have a great spec-kit example? We'd love to include it!

1. Fork the repository
2. Create your example in `examples/your-example/`
3. Include complete specifications, plans, and code
4. Add a comprehensive README
5. Submit a pull request

See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

## Additional Resources

- [Comprehensive Tutorials](../TUTORIALS.md) - Step-by-step guides
- [Main Documentation](../README.md) - Full project documentation
- [GitHub Spec-Kit](https://github.com/github/spec-kit) - Original methodology
- [MCP Documentation](https://modelcontextprotocol.io/) - Protocol details

## Support

- **Issues**: [GitHub Issues](https://github.com/lsendel/spec-kit-mcp/issues)
- **Discussions**: [GitHub Discussions](https://github.com/lsendel/spec-kit-mcp/discussions)
- **Documentation**: [Full Docs](https://github.com/lsendel/spec-kit-mcp)

---

**Happy Learning!** 🚀
