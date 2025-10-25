# Spec-Kit MCP Usage Guide

This guide shows how to use the spec-kit MCP server in your development workflow.

## Installation

### Step 1: Install Prerequisites

```bash
# Install spec-kit CLI
uv tool install specify-cli

# Verify installation
specify --version
```

### Step 2: Install Spec-Kit MCP

**Option A: Via Cargo (recommended for development)**

```bash
cargo install spec-kit-mcp
```

**Option B: Via NPX (when published)**

```bash
# No installation needed! Just configure in .mcp.json
npx @speckit/mcp@latest
```

## Configuration

### For Claude Code

Add to your project's `.mcp.json`:

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

Or if using npx:

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

## Complete Workflow Example

Here's a real-world example of using spec-kit MCP to build a new feature:

### Scenario: Add a New "Export" Feature to pmatinit Calculator

#### Step 1: Initialize

```
You: I want to add a data export feature to the pmatinit calculator. Let's start with spec-kit.

Claude: I'll help you use spec-driven development. Let me initialize a spec-kit project.

[Uses speckit_init tool]
✓ Initialized spec-kit project "pmatinit-export-feature"
```

#### Step 2: Constitution

```
You: Create a constitution emphasizing:
- Consistency with existing pmatinit architecture
- User-friendly export formats (CSV, JSON)
- Maintain high performance
- Follow Rust best practices

Claude: [Uses speckit_constitution tool with these principles]

✓ Created ./speckit.constitution with:
  - Core principles aligned with pmatinit
  - Technical constraints (Rust 2021, serde for serialization)
  - Quality standards (tests, documentation)
```

#### Step 3: Specify Requirements

```
You: Specify requirements for exporting calculator history and variables

Claude: [Uses speckit_specify tool]

✓ Created ./speckit.specify with:

## Requirements
1. Export calculator history to CSV format
2. Export calculator history to JSON format
3. Export current variables to JSON
4. Export session summary
5. Allow filtering by date/time range

## User Stories
- As a data analyst, I want to export calculator history to CSV
  so that I can analyze calculations in Excel

- As a developer, I want to export variables to JSON
  so that I can restore calculator state

- As a student, I want to export session summaries
  so that I can review my work later
```

#### Step 4: Create Technical Plan

```
You: Create a technical plan using the existing pmatinit architecture

Claude: [Uses speckit_plan tool]

✓ Created ./speckit.plan with:

## Architecture
- Add new `export` module under `src/`
- Use serde for JSON serialization
- Use csv crate for CSV export
- Integrate with existing REPL commands

## Modules
1. src/export/mod.rs - Export manager
2. src/export/csv.rs - CSV formatter
3. src/export/json.rs - JSON formatter
4. src/repl/export_commands.rs - REPL commands

## Implementation Approach
- Phase 1: JSON export (simpler)
- Phase 2: CSV export
- Phase 3: REPL integration
- Phase 4: Testing & docs
```

#### Step 5: Generate Tasks

```
You: Generate detailed tasks

Claude: [Uses speckit_tasks tool]

✓ Created ./speckit.tasks with 23 actionable items:

### Phase 1: JSON Export (Tasks 1-8)
1. Create src/export/mod.rs module structure
2. Define ExportFormat enum
3. Implement HistoryEntry serialization
4. Implement Variable serialization
5. Add export_json() function
6. Write unit tests for JSON export
7. Test with sample data
8. Document JSON export API

### Phase 2: CSV Export (Tasks 9-14)
9. Add csv dependency to Cargo.toml
10. Implement CSV formatter in src/export/csv.rs
11. Handle header row generation
12. Handle data row generation
13. Write unit tests for CSV export
14. Document CSV export API

### Phase 3: REPL Integration (Tasks 15-19)
15. Add export commands to REPL
16. Implement command parsing
17. Add file path validation
18. Add success/error messages
19. Update help text

### Phase 4: Testing & Docs (Tasks 20-23)
20. Integration tests
21. Update README.md
22. Add usage examples
23. Update CHANGELOG.md
```

#### Step 6: Implementation

```
You: Let's implement these tasks

Claude: I'll help implement each task. Starting with Phase 1...

[Proceeds to implement the code following the tasks]
```

---

## Tool Reference

### speckit_init

Initialize a new spec-kit project.

**When to use**: Start of any new feature or project

**Example**:
```json
{
  "project_name": "my-feature",
  "project_path": "./features/my-feature"
}
```

### speckit_constitution

Define governing principles and constraints.

**When to use**: After initialization, before specification

**Example**:
```json
{
  "principles": "Simplicity, Performance, Security, Testability",
  "constraints": "Must be backward compatible with v1.x API",
  "output_path": "./speckit.constitution"
}
```

### speckit_specify

Define what to build (requirements and user stories).

**When to use**: After constitution, before planning

**Example**:
```json
{
  "requirements": "Add OAuth2 authentication with Google and GitHub providers",
  "user_stories": "As a user, I want to login with Google so that I don't need another password",
  "output_path": "./speckit.specify"
}
```

### speckit_plan

Create a technical implementation plan.

**When to use**: After specification, before task breakdown

**Example**:
```json
{
  "spec_file": "./speckit.specify",
  "tech_stack": "Rust + OAuth2 crate + Axum web framework",
  "output_path": "./speckit.plan"
}
```

### speckit_tasks

Generate actionable task list.

**When to use**: After planning, before implementation

**Example**:
```json
{
  "plan_file": "./speckit.plan",
  "breakdown_level": "detailed",
  "output_path": "./speckit.tasks"
}
```

---

## Best Practices

### 1. Start with Why

Always begin with the constitution. Define principles before diving into details.

```
❌ Bad: "Let's add OAuth"
✅ Good: "Let's define security principles, then specify OAuth requirements"
```

### 2. Be Specific in Requirements

Use concrete examples and user stories.

```
❌ Bad: "Add export feature"
✅ Good: "As a data analyst, I want to export to CSV with headers, timestamps, and calculated results"
```

### 3. Iterate on Specifications

Don't be afraid to refine specs based on learnings.

```
You: After reviewing the plan, I realize we also need Excel export
Claude: [Updates speckit.specify with new requirement]
Claude: [Updates speckit.plan with Excel support]
Claude: [Regenerates speckit.tasks with new tasks]
```

### 4. Keep Artifacts in Version Control

```bash
git add speckit.*
git commit -m "feat: add export feature specification"
```

### 5. Review Before Implementation

Share specs with team/stakeholders before coding.

```
You: Review the spec with the team and incorporate feedback
Claude: [Updates artifacts based on feedback]
```

---

## Integration with Other MCPs

Spec-kit MCP works great alongside other MCPs:

### With PMAT (Code Quality)

```
You: Use spec-kit to plan the feature, then run PMAT analysis

Claude: [Uses speckit tools to create specs]
Claude: [Uses PMAT tools to analyze code quality]
Claude: [Incorporates quality metrics into implementation]
```

### With Playwright (Testing)

```
You: After implementing the export UI, use Playwright to test it

Claude: [Implements feature using spec-kit workflow]
Claude: [Uses Playwright MCP to create browser tests]
```

---

## Troubleshooting

### "spec-kit CLI not found"

The MCP server checks for spec-kit at startup:

```
Error: spec-kit CLI not found!
Please install it with: uv tool install specify-cli
```

**Solution**:
```bash
uv tool install specify-cli
```

### "Timeout waiting for spec-kit command"

Default timeout is 300 seconds. For longer operations:

```bash
spec-kit-mcp --timeout 600
```

### Debugging

Enable debug logging:

```bash
spec-kit-mcp --log-level debug
```

Logs go to stderr, so they don't interfere with MCP protocol (stdout).

---

## Tips & Tricks

### 1. Use Git Branches for Specs

```bash
git checkout -b feature/export-spec
# Create specs with spec-kit
git commit -m "spec: export feature specification"
git push origin feature/export-spec
# Share for review
```

### 2. Automate with Scripts

```bash
#!/bin/bash
# spec-workflow.sh
specify init "$1"
# ... automation ...
```

### 3. Template Reuse

Keep successful specs as templates:

```bash
cp old-feature/speckit.constitution new-feature/
# Modify for new context
```

### 4. Progressive Detail

Start high-level, add detail iteratively:

```
Iteration 1: High-level spec (1 page)
Iteration 2: Detailed spec (5 pages)
Iteration 3: Implementation-ready (15 pages with diagrams)
```

---

## FAQ

**Q: Do I need to use all 5 tools for every feature?**
A: No, but it's recommended for complex features. Simple bug fixes may not need full spec-kit workflow.

**Q: Can I modify specs after starting implementation?**
A: Yes! Specs evolve. Update them as you learn.

**Q: How detailed should requirements be?**
A: Detailed enough to guide implementation but not so detailed they constrain solutions.

**Q: Should I check specs into version control?**
A: Yes! Specs are valuable documentation.

---

## Real-World Examples

### Example 1: New Calculator Feature

See the pmatinit export feature example above.

### Example 2: Bug Fix with Root Cause

```
You: Use spec-kit to document the bug and solution approach

Claude: [Uses speckit_specify to document bug]
Claude: [Uses speckit_plan to outline fix approach]
Claude: [Implements fix following the plan]
```

### Example 3: Refactoring

```
You: We need to refactor the parser. Use spec-kit to plan it.

Claude: [Creates constitution with refactoring principles]
Claude: [Specifies refactoring requirements]
Claude: [Plans phased refactoring approach]
Claude: [Generates tasks for safe refactoring]
```

---

## Resources

- **Spec-Kit GitHub**: https://github.com/github/spec-kit
- **MCP Documentation**: https://modelcontextprotocol.io/
- **This MCP Server**: https://github.com/yourusername/spec-kit-mcp
- **Examples**: See `examples/` directory

---

**Happy Spec-Driven Development!** 🚀
