# Todo CLI - Spec-Kit Example

A simple command-line task manager built using spec-driven development with the Spec-Kit MCP Server.

## What You'll Learn

- ✅ Initializing a spec-kit project
- ✅ Writing clear specifications
- ✅ Creating implementation plans
- ✅ Generating task checklists
- ✅ Implementing with AI assistance
- ✅ Testing and quality assurance

## Project Overview

A command-line application that allows users to:
- Add new tasks
- List all tasks
- Complete tasks
- Delete tasks
- Filter tasks by status

## Prerequisites

- Node.js 18+ (or Python 3.8+, or Rust 1.70+)
- spec-kit-mcp installed
- An AI coding assistant (Claude, Cursor, or Windsurf)

## Quick Start

### Option 1: Follow Along (Recommended for Learning)

Start from scratch and build the application step by step:

```bash
# Create a new directory
mkdir my-todo-cli
cd my-todo-cli

# Open in your AI assistant
code .  # or cursor . or windsurf .
```

Then follow the [Step-by-Step Guide](#step-by-step-guide) below.

### Option 2: Explore the Complete Example

```bash
# Navigate to this example
cd examples/todo-cli

# Install dependencies
npm install  # or pip install -r requirements.txt, or cargo build

# Run the application
npm start add "My first task"
npm start list
```

## Step-by-Step Guide

### Step 1: Project Initialization

**Prompt your AI assistant**:
```
Use the speckit_init tool to create a new project with:
- Name: todo-cli
- Type: CLI Application
- Language: TypeScript (or Python/Rust)
- Description: A command-line task management application
```

**Expected files created**:
- `.speckit/CONSTITUTION.md`
- Basic project structure

### Step 2: Review the Constitution

Open `.speckit/CONSTITUTION.md` and review the generated project principles.

**Key sections**:
- Core Purpose
- Technical Principles
- Code Quality Standards
- Testing Requirements

### Step 3: Create Feature Specification

**Prompt your AI assistant**:
```
Use the speckit_specify tool to create a specification for:
- Feature: "add-task"
- Description: "Users can add new tasks with a title and optional description"

Include:
- User stories
- Acceptance criteria
- API/CLI interface
- Data model
- Error handling
```

**Generated**: `.speckit/specs/add-task.md`

### Step 4: Review and Refine

Open the specification and ensure it covers:
- [ ] Clear user story
- [ ] Specific acceptance criteria
- [ ] CLI command syntax
- [ ] Data storage format
- [ ] Error cases
- [ ] Success messages

If anything is unclear, use `speckit_clarify`:

**Prompt**:
```
Use the speckit_clarify tool on add-task.md to clarify:
- Should tasks have due dates or priorities?
- What storage format should we use (JSON, SQLite)?
- Should there be task categories/tags?
```

### Step 5: Create More Specifications

Repeat for other features:

**Prompt**:
```
Use speckit_specify to create specifications for:
1. list-tasks - Display all tasks with filtering
2. complete-task - Mark tasks as complete
3. delete-task - Remove tasks
```

### Step 6: Create Implementation Plan

**Prompt**:
```
Use the speckit_plan tool to create an implementation plan from all specifications.
Consider:
- Implementation order (dependencies)
- Shared components (storage, CLI parser)
- Testing strategy
```

**Generated**: `.speckit/plans/todo-cli-plan.md`

### Step 7: Generate Task Checklist

**Prompt**:
```
Use the speckit_tasks tool to generate a detailed task checklist from the plan.
Output: .speckit/tasks/todo-cli-tasks.md
```

**Example tasks**:
```markdown
## Foundation
- [ ] Set up project structure
- [ ] Create Task data model
- [ ] Implement JSON storage module
- [ ] Add error handling utilities

## Feature: Add Task
- [ ] Implement add command parser
- [ ] Create task validation
- [ ] Add task to storage
- [ ] Display confirmation message
- [ ] Write unit tests

## Feature: List Tasks
- [ ] Implement list command parser
- [ ] Add filtering logic
- [ ] Format output display
- [ ] Write unit tests

... (continues)
```

### Step 8: Implement Features

**Prompt**:
```
Use the speckit_implement tool with:
- Task file: .speckit/tasks/todo-cli-tasks.md
- Context: This is a beginner-friendly CLI app, prioritize simplicity and clear code
- Output directory: ./src

Please implement the features systematically, following the task order.
```

### Step 9: Run Tests

```bash
# Run the test suite
npm test  # or pytest, or cargo test
```

### Step 10: Quality Check

**Prompt**:
```
Use the speckit_analyze tool on the codebase to check:
- Code quality metrics
- Test coverage
- Constitution compliance
- Potential improvements
```

### Step 11: Create Review Checklist

**Prompt**:
```
Use the speckit_checklist tool to create a final review checklist ensuring:
- All acceptance criteria met
- Tests passing
- Documentation complete
- Ready for release
```

### Step 12: Try the Application

```bash
# Add some tasks
./todo add "Learn spec-kit methodology"
./todo add "Build a CLI app" --description "Using TypeScript"
./todo add "Write tests"

# List all tasks
./todo list

# Complete a task
./todo complete 1

# List only incomplete tasks
./todo list --status incomplete

# Delete a task
./todo delete 2
```

## Project Structure

```
todo-cli/
├── README.md
├── package.json (or requirements.txt/Cargo.toml)
├── .speckit/
│   ├── CONSTITUTION.md
│   ├── specs/
│   │   ├── add-task.md
│   │   ├── list-tasks.md
│   │   ├── complete-task.md
│   │   └── delete-task.md
│   ├── plans/
│   │   └── todo-cli-plan.md
│   └── tasks/
│       └── todo-cli-tasks.md
├── src/
│   ├── index.ts (or main.py/main.rs)
│   ├── models/
│   │   └── task.ts
│   ├── storage/
│   │   └── json-storage.ts
│   ├── commands/
│   │   ├── add.ts
│   │   ├── list.ts
│   │   ├── complete.ts
│   │   └── delete.ts
│   └── utils/
│       └── cli-parser.ts
├── tests/
│   ├── task.test.ts
│   ├── storage.test.ts
│   └── commands.test.ts
└── data/
    └── tasks.json
```

## Key Files

### `.speckit/specs/add-task.md`

```markdown
# Feature: Add Task

## User Story
As a user, I want to add new tasks with titles and optional descriptions so that I can track my work.

## Acceptance Criteria
- [ ] User can run `todo add "Task title"`
- [ ] User can add optional description with `--description` flag
- [ ] Tasks are assigned unique IDs automatically
- [ ] Tasks are saved to persistent storage
- [ ] User sees confirmation with task ID
- [ ] Empty titles are rejected with error message

## CLI Interface
\`\`\`bash
todo add <title> [--description <desc>]
\`\`\`

## Data Model
\`\`\`typescript
interface Task {
  id: number;
  title: string;
  description?: string;
  status: 'incomplete' | 'complete';
  createdAt: Date;
  completedAt?: Date;
}
\`\`\`

## Success Output
\`\`\`
✅ Task added successfully (ID: 1)
   "Learn spec-kit methodology"
\`\`\`

## Error Cases
- Empty title: "Error: Task title cannot be empty"
- Storage failure: "Error: Failed to save task"
```

### `src/models/task.ts` (TypeScript example)

```typescript
export interface Task {
  id: number;
  title: string;
  description?: string;
  status: 'incomplete' | 'complete';
  createdAt: Date;
  completedAt?: Date;
}

export class TaskValidator {
  static validate(title: string): { valid: boolean; error?: string } {
    if (!title || title.trim().length === 0) {
      return { valid: false, error: 'Task title cannot be empty' };
    }
    if (title.length > 200) {
      return { valid: false, error: 'Task title too long (max 200 characters)' };
    }
    return { valid: true };
  }
}
```

### `src/storage/json-storage.ts`

```typescript
import * as fs from 'fs/promises';
import { Task } from '../models/task';

export class JsonStorage {
  constructor(private filePath: string = './data/tasks.json') {}

  async load(): Promise<Task[]> {
    try {
      const data = await fs.readFile(this.filePath, 'utf-8');
      return JSON.parse(data);
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code === 'ENOENT') {
        return [];
      }
      throw error;
    }
  }

  async save(tasks: Task[]): Promise<void> {
    const data = JSON.stringify(tasks, null, 2);
    await fs.writeFile(this.filePath, data, 'utf-8');
  }

  async add(task: Task): Promise<void> {
    const tasks = await this.load();
    tasks.push(task);
    await this.save(tasks);
  }
}
```

### `tests/task.test.ts`

```typescript
import { describe, it, expect } from 'vitest';
import { TaskValidator } from '../src/models/task';

describe('TaskValidator', () => {
  it('should accept valid task titles', () => {
    const result = TaskValidator.validate('Valid task title');
    expect(result.valid).toBe(true);
    expect(result.error).toBeUndefined();
  });

  it('should reject empty titles', () => {
    const result = TaskValidator.validate('');
    expect(result.valid).toBe(false);
    expect(result.error).toContain('cannot be empty');
  });

  it('should reject whitespace-only titles', () => {
    const result = TaskValidator.validate('   ');
    expect(result.valid).toBe(false);
    expect(result.error).toContain('cannot be empty');
  });

  it('should reject titles that are too long', () => {
    const longTitle = 'a'.repeat(201);
    const result = TaskValidator.validate(longTitle);
    expect(result.valid).toBe(false);
    expect(result.error).toContain('too long');
  });
});
```

## Testing Strategy

### Unit Tests
- Task validation logic
- Storage operations
- Command parsing
- Data model

### Integration Tests
- Complete workflows (add → list → complete → delete)
- Storage persistence
- Error handling

### Running Tests

```bash
# Run all tests
npm test

# Run with coverage
npm test -- --coverage

# Run specific test file
npm test -- task.test.ts

# Watch mode
npm test -- --watch
```

## Common Issues

### Issue: Storage file not found

**Error**: `ENOENT: no such file or directory`

**Solution**: Ensure the `data/` directory exists
```bash
mkdir -p data
```

### Issue: Tests failing

**Solution**: Check that all dependencies are installed
```bash
npm install
```

### Issue: Command not found

**Solution**: Make the CLI executable
```bash
chmod +x todo
# or use: node src/index.js instead of ./todo
```

## Extensions

Want to add more features? Try these:

1. **Due Dates**: Add `dueDate` field and filtering
2. **Priorities**: Add priority levels (high, medium, low)
3. **Categories**: Add task categories/tags
4. **Search**: Full-text search across tasks
5. **Export**: Export tasks to CSV/JSON
6. **Recurring Tasks**: Support for recurring tasks

For each extension:
1. Create a specification with `speckit_specify`
2. Generate plan with `speckit_plan`
3. Create tasks with `speckit_tasks`
4. Implement with `speckit_implement`

## Learning Points

After completing this example, you should understand:

✅ **Specification-First Development**
- Writing clear, actionable specifications
- Defining acceptance criteria
- Documenting interfaces

✅ **Planning and Task Breakdown**
- Breaking features into tasks
- Identifying dependencies
- Prioritizing work

✅ **AI-Assisted Implementation**
- Using MCP tools effectively
- Providing context to AI
- Verifying AI-generated code

✅ **Quality Assurance**
- Testing strategies
- Code quality checks
- Constitution compliance

## Next Steps

1. ✅ Complete this Todo CLI example
2. → Move to [Blog API](../blog-api/) for API development
3. → Try [Web App](../web-app/) for full-stack development
4. → Challenge yourself with [Refactoring](../refactoring/)

## Resources

- [Back to Examples](../README.md)
- [Full Tutorials](../../TUTORIALS.md)
- [Main Documentation](../../README.md)

---

**Questions?** Open an issue at [GitHub Issues](https://github.com/lsendel/spec-kit-mcp/issues)
