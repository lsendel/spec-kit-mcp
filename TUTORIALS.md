# Spec-Kit MCP Tutorials

Comprehensive step-by-step tutorials for using the Spec-Kit MCP Server with AI coding assistants.

## Table of Contents

- [Getting Started](#getting-started)
- [Tutorial 1: Your First Spec-Kit Project](#tutorial-1-your-first-spec-kit-project)
- [Tutorial 2: Building a REST API with Spec-Kit](#tutorial-2-building-a-rest-api-with-spec-kit)
- [Tutorial 3: Advanced Workflows - Refactoring Legacy Code](#tutorial-3-advanced-workflows---refactoring-legacy-code)
- [Tutorial 4: Team Collaboration with Spec-Kit](#tutorial-4-team-collaboration-with-spec-kit)
- [Tutorial 5: Integrating with CI/CD](#tutorial-5-integrating-with-cicd)

---

## Getting Started

### Prerequisites

- An AI coding assistant (Claude Code, Cursor, Windsurf, or VS Code with MCP support)
- Node.js 18+ or Rust toolchain
- Git installed
- Basic understanding of software development workflows

### Installation

Choose your preferred installation method:

```bash
# Option 1: Using npx (no installation needed)
npx @lsendel/spec-kit-mcp

# Option 2: Install via cargo
cargo install spec-kit-mcp

# Option 3: Install via npm globally
npm install -g @lsendel/spec-kit-mcp
```

### Verify Installation

```bash
# If installed globally
spec-kit-mcp --help

# If using npx
npx @lsendel/spec-kit-mcp --help
```

---

## Tutorial 1: Your First Spec-Kit Project

**Goal**: Create a simple task management CLI application using the spec-kit methodology.

**Time**: 20-30 minutes

### Step 1: Project Initialization

Create a new project directory and initialize spec-kit:

```bash
mkdir task-manager
cd task-manager
git init
```

### Step 2: Open in Your AI Assistant

Open the project in your preferred AI coding assistant:

```bash
# For VS Code / Cursor
code .

# For Windsurf
windsurf .
```

### Step 3: Use the speckit_check Tool

In your AI assistant, run the check tool to verify your environment:

**Prompt to AI**:
```
Use the speckit_check tool to verify my development environment is ready
```

**Expected Output**:
- ✅ Git is available
- ✅ AI coding assistant detected
- Status report on your environment

### Step 4: Initialize Project Constitution

**Prompt to AI**:
```
Use the speckit_init tool to create a new project with:
- Name: task-manager
- Type: CLI Application
- Language: Python (or your preferred language)
- Description: A command-line task management application
```

**What happens**:
- Creates `.speckit/` directory
- Generates `CONSTITUTION.md` with project principles
- Sets up initial project structure

### Step 5: Review the Constitution

Open `.speckit/CONSTITUTION.md` and review the generated project principles:

```markdown
# Task Manager Constitution

## Core Purpose
A command-line task management application

## Technical Principles
- Clean, maintainable code
- Comprehensive testing
- Clear documentation
...
```

**Customize if needed**:
- Add your team's coding standards
- Include architectural decisions
- Define testing requirements

### Step 6: Create Initial Specification

**Prompt to AI**:
```
Use the speckit_specify tool to create a specification for the first feature:
- Feature: "Add task command"
- Description: "Users should be able to add new tasks with a title and optional description"
- Include acceptance criteria
```

**Generated**: `.speckit/specs/add-task.md`

### Step 7: Review and Refine the Spec

Open the generated specification and review it:

```markdown
# Feature: Add Task Command

## User Story
As a user, I want to add new tasks with titles and descriptions...

## Acceptance Criteria
- [ ] User can run `task add "Task title"`
- [ ] User can provide optional description with --description flag
- [ ] Tasks are persisted to local storage
- [ ] User receives confirmation message

## Technical Details
...
```

**Prompt to AI** (if refinement needed):
```
Use the speckit_clarify tool on add-task.md to clarify:
- What storage format should we use (JSON, SQLite, etc.)?
- Should tasks have due dates?
- What about task priorities?
```

### Step 8: Create Implementation Plan

**Prompt to AI**:
```
Use the speckit_plan tool to create an implementation plan from add-task.md
```

**Generated**: `.speckit/plans/add-task-plan.md`

This breaks down the feature into concrete implementation tasks.

### Step 9: Generate Task List

**Prompt to AI**:
```
Use the speckit_tasks tool to generate a task checklist from add-task-plan.md
Output file: .speckit/tasks/add-task-tasks.md
```

**Generated**: A checklist like:
```markdown
- [ ] Create Task data structure
- [ ] Implement JSON storage module
- [ ] Create CLI argument parser
- [ ] Implement add command
- [ ] Add confirmation message
- [ ] Write unit tests
- [ ] Update documentation
```

### Step 10: Implement the Feature

**Prompt to AI**:
```
Use the speckit_implement tool with:
- Task file: .speckit/tasks/add-task-tasks.md
- Context: This is our first feature, focus on simplicity and testing
- Output directory: ./src

Please implement the feature according to the specification
```

**What happens**:
- AI generates the code based on specs
- Tests are created
- Documentation is updated
- Each task is completed systematically

### Step 11: Verify Implementation

Run your tests and verify the implementation:

```bash
# For Python example
python -m pytest

# Try the CLI
python task_manager.py add "My first task" --description "Learn spec-kit"
```

### Step 12: Analyze Quality

**Prompt to AI**:
```
Use the speckit_analyze tool on ./src to check:
- Code quality metrics
- Test coverage
- Compliance with constitution
```

### Step 13: Create Checklist for Review

**Prompt to AI**:
```
Use the speckit_checklist tool to create a review checklist for this feature
```

**Generated**: A checklist ensuring:
- All acceptance criteria met
- Tests passing
- Documentation complete
- Code follows constitution principles

### Step 14: Commit Your Work

```bash
git add .
git commit -m "feat: Add task command

Implemented task addition with persistence
- JSON storage
- CLI interface
- Comprehensive tests
- Documentation

Closes #1"
```

### Congratulations!

You've completed your first spec-kit workflow! You've learned to:

1. ✅ Initialize a spec-kit project
2. ✅ Create specifications
3. ✅ Generate implementation plans
4. ✅ Create task lists
5. ✅ Implement with AI assistance
6. ✅ Analyze and verify quality

---

## Tutorial 2: Building a REST API with Spec-Kit

**Goal**: Build a REST API for a blog platform using spec-driven development.

**Time**: 45-60 minutes

### Project Overview

We'll build a simple blog API with:
- User authentication
- Post creation and management
- Comments on posts
- RESTful endpoints

### Step 1: Initialize the Project

```bash
mkdir blog-api
cd blog-api
git init
```

**Prompt to AI**:
```
Use speckit_init to create a new project:
- Name: blog-api
- Type: REST API
- Language: Rust (or Node.js/Python)
- Description: RESTful API for a blog platform with authentication
```

### Step 2: Define the Constitution

**Prompt to AI**:
```
Use speckit_constitution tool to update our constitution with:
- API design principles (RESTful, versioned)
- Security requirements (authentication, input validation)
- Testing standards (unit tests, integration tests, API tests)
- Documentation standards (OpenAPI/Swagger)
```

### Step 3: Create High-Level Architecture Spec

**Prompt to AI**:
```
Use speckit_specify to create an architecture specification:
- Feature name: "api-architecture"
- Include: Database choice, authentication strategy, API structure, deployment considerations
```

### Step 4: Break Down into Features

Create specifications for each major feature:

**Prompt to AI**:
```
Use speckit_specify to create specifications for:
1. User authentication (JWT-based)
2. Post management (CRUD operations)
3. Comment management
4. User profiles

For each, include:
- API endpoints
- Request/response formats
- Authentication requirements
- Validation rules
```

### Step 5: Prioritize and Plan

**Prompt to AI**:
```
Use speckit_plan tool to create implementation plans for each feature in priority order:
1. User authentication (foundation)
2. Post management (core feature)
3. User profiles
4. Comments
```

### Step 6: Start with Authentication

**Prompt to AI**:
```
Use speckit_tasks to generate tasks for the authentication feature from .speckit/plans/authentication-plan.md
```

### Step 7: Implement Authentication

**Prompt to AI**:
```
Use speckit_implement with:
- Task file: .speckit/tasks/authentication-tasks.md
- Context: Using JWT tokens, bcrypt for passwords, in-memory user store initially
- Output: ./src/auth/

Implement the authentication system
```

### Step 8: Test Authentication

Write and run tests for the authentication endpoints:

```bash
# Example curl tests
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","email":"test@example.com","password":"secure123"}'

curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"secure123"}'
```

### Step 9: Continue with Post Management

Repeat the process for posts:

**Prompt to AI**:
```
Now let's implement post management:
1. Use speckit_tasks for post feature
2. Use speckit_implement with authentication context
3. Ensure all endpoints require authentication
```

### Step 10: Add Integration Tests

**Prompt to AI**:
```
Create integration tests that:
- Test full user registration → login → post creation flow
- Test authentication failures
- Test authorization (users can only edit their own posts)
```

### Step 11: Generate API Documentation

**Prompt to AI**:
```
Based on our implementations:
1. Generate OpenAPI/Swagger documentation
2. Create API usage examples
3. Document error codes and responses
```

### Step 12: Quality Analysis

**Prompt to AI**:
```
Use speckit_analyze on the entire codebase to check:
- API consistency
- Error handling
- Security best practices
- Test coverage
```

### Step 13: Create Deployment Plan

**Prompt to AI**:
```
Use speckit_specify to create a deployment specification:
- Environment configuration
- Database migrations
- Monitoring and logging
- CI/CD pipeline
```

### Result

You now have a production-ready REST API built with:
- ✅ Clear specifications for all features
- ✅ Systematic implementation
- ✅ Comprehensive testing
- ✅ Complete documentation
- ✅ Security best practices

---

## Tutorial 3: Advanced Workflows - Refactoring Legacy Code

**Goal**: Use spec-kit to systematically refactor and modernize a legacy codebase.

**Time**: 60-90 minutes

### Scenario

You have an old monolithic application that needs refactoring. The spec-kit methodology will help you:
1. Document the current system
2. Define the target architecture
3. Create a migration plan
4. Execute refactoring systematically

### Step 1: Analyze the Legacy Code

**Prompt to AI**:
```
Use speckit_analyze on the legacy codebase at ./legacy-app to:
- Identify technical debt
- Map dependencies
- Find code smells
- Assess test coverage
```

### Step 2: Document Current State

**Prompt to AI**:
```
Use speckit_specify to create a "current-state" specification:
- Document the existing architecture
- List known issues and limitations
- Identify pain points
- Map dependencies
```

### Step 3: Define Target Architecture

**Prompt to AI**:
```
Use speckit_specify to create a "target-architecture" specification:
- Modern architecture patterns (e.g., microservices, clean architecture)
- Technology stack upgrades
- Performance improvements
- Maintainability goals
```

### Step 4: Create Migration Strategy

**Prompt to AI**:
```
Use speckit_plan to create a migration plan from current-state to target-architecture:
- Break down into phases
- Identify risks
- Plan for backward compatibility
- Define rollback strategies
```

### Step 5: Strangler Fig Pattern

Implement refactoring using the strangler fig pattern:

**Prompt to AI**:
```
Use speckit_specify for phase 1 - "Extract User Service":
- Extract user management from monolith
- Create new microservice
- Implement API contract
- Add integration layer
```

### Step 6: Implement with Testing

**Prompt to AI**:
```
Use speckit_tasks to create tasks for user service extraction

For each task, ensure:
- Existing functionality preserved
- Comprehensive tests
- Monitoring in place
- Gradual rollout plan
```

### Step 7: Validate Migration

**Prompt to AI**:
```
Use speckit_checklist to create validation checklist:
- All existing tests still pass
- New tests for extracted service
- Performance benchmarks
- No regressions
```

### Step 8: Document Changes

**Prompt to AI**:
```
Update CONSTITUTION.md to reflect:
- New architectural decisions
- Migration lessons learned
- Updated development standards
```

### Step 9: Repeat for Next Service

Continue the strangler fig pattern for each component:
- Extract authentication service
- Extract payment service
- Extract notification service
- etc.

### Best Practices Learned

1. **Document everything**: The current state is just as important as the target
2. **Small iterations**: Break refactoring into small, deployable units
3. **Test thoroughly**: Ensure no regressions
4. **Monitor continuously**: Watch for performance impacts
5. **Communicate changes**: Keep team and stakeholders informed

---

## Tutorial 4: Team Collaboration with Spec-Kit

**Goal**: Set up spec-kit workflows for team development.

**Time**: 30-45 minutes

### Setting Up Team Workflows

### Step 1: Establish Team Constitution

**Prompt to AI**:
```
Use speckit_constitution to create our team constitution:
- Code review process
- Testing standards
- Documentation requirements
- Git workflow (branch naming, commit messages)
- Pull request template
```

### Step 2: Create Specification Template

Create a team specification template in `.speckit/templates/feature-spec.md`:

```markdown
# Feature: [Name]

## Author
[Your name]

## Date
[YYYY-MM-DD]

## Problem Statement
What problem does this feature solve?

## User Stories
As a [user type], I want [goal] so that [benefit]

## Acceptance Criteria
- [ ] Criterion 1
- [ ] Criterion 2

## Technical Approach
How will we implement this?

## Dependencies
What other features/systems does this depend on?

## Testing Strategy
How will we verify this works?

## Security Considerations
Any security implications?

## Performance Considerations
Any performance implications?

## Documentation Needed
What documentation must be created/updated?

## Open Questions
What needs clarification?
```

### Step 3: Specification Review Process

Establish a spec review process:

1. **Author creates spec** using `speckit_specify`
2. **Request clarification** using `speckit_clarify` if needed
3. **Team reviews** specification before implementation
4. **Get approval** from tech lead / architect
5. **Create plan** using `speckit_plan`
6. **Generate tasks** using `speckit_tasks`
7. **Implement** using `speckit_implement`

### Step 4: Pull Request Workflow

**Before opening PR**:

**Prompt to AI**:
```
Use speckit_checklist to generate PR checklist:
- All acceptance criteria met
- Tests passing
- Documentation updated
- Constitution compliance
- No technical debt introduced
```

### Step 5: Code Review Integration

**For reviewers**, create a review checklist:

**Prompt to AI**:
```
Use speckit_analyze on the PR changes to check:
- Specification compliance
- Code quality metrics
- Test coverage
- Documentation completeness
```

### Step 6: Onboarding New Team Members

Create an onboarding spec:

**Prompt to AI**:
```
Use speckit_specify to create "team-onboarding" specification:
- Setup checklist
- Codebase overview
- Development workflow
- Tool configuration
- First task suggestions
```

### Step 7: Sprint Planning

Use spec-kit in sprint planning:

**Prompt to AI**:
```
For each story in the sprint:
1. Create specification using speckit_specify
2. Generate implementation plan using speckit_plan
3. Estimate complexity
4. Identify dependencies
```

### Example: Feature Branch Workflow

```bash
# 1. Create feature branch
git checkout -b feature/user-notifications

# 2. Create specification
# Prompt AI: "Use speckit_specify to create notification spec"

# 3. Review and refine spec
git add .speckit/specs/user-notifications.md
git commit -m "spec: Add user notifications specification"
git push origin feature/user-notifications

# 4. Open spec review PR
# Team reviews, discusses, approves

# 5. Create implementation plan
# Prompt AI: "Use speckit_plan for user-notifications"

# 6. Generate tasks
# Prompt AI: "Use speckit_tasks for notification plan"

# 7. Implement feature
# Prompt AI: "Use speckit_implement for notifications"

# 8. Run quality checks
# Prompt AI: "Use speckit_analyze on notification code"

# 9. Create checklist and verify
# Prompt AI: "Use speckit_checklist for notifications"

# 10. Open implementation PR
git add .
git commit -m "feat: Implement user notifications

Implemented email and in-app notifications
- Real-time delivery via WebSocket
- Email templates
- Notification preferences
- Comprehensive tests

Implements #42"
git push origin feature/user-notifications
```

---

## Tutorial 5: Integrating with CI/CD

**Goal**: Automate spec-kit workflows in your CI/CD pipeline.

**Time**: 45-60 minutes

### GitHub Actions Integration

Create `.github/workflows/spec-kit-validation.yml`:

```yaml
name: Spec-Kit Validation

on:
  pull_request:
    branches: [main, develop]
  push:
    branches: [main, develop]

jobs:
  validate-specs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install spec-kit-mcp
        run: cargo install spec-kit-mcp

      - name: Check for specification
        run: |
          # Ensure PR has associated specification
          if [ ! -d ".speckit/specs" ]; then
            echo "Error: No specifications found"
            exit 1
          fi

      - name: Validate constitution compliance
        run: |
          spec-kit-mcp analyze --check-constitution

      - name: Check implementation completeness
        run: |
          spec-kit-mcp checklist --verify-all

      - name: Generate quality report
        run: |
          spec-kit-mcp analyze --output quality-report.md

      - name: Comment quality report on PR
        uses: actions/github-script@v6
        with:
          script: |
            const fs = require('fs');
            const report = fs.readFileSync('quality-report.md', 'utf8');
            github.rest.issues.createComment({
              issue_number: context.issue.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: report
            });
```

### Pre-commit Hooks

Create `.git/hooks/pre-commit`:

```bash
#!/bin/bash

echo "Running spec-kit validation..."

# Check if changes include implementation without spec
if git diff --cached --name-only | grep -q "src/"; then
    if [ ! -d ".speckit/specs" ] || [ -z "$(ls -A .speckit/specs 2>/dev/null)" ]; then
        echo "Error: Implementation changes require a specification"
        echo "Please create a spec using: speckit_specify"
        exit 1
    fi
fi

# Validate constitution compliance
if ! spec-kit-mcp analyze --quick-check; then
    echo "Error: Code doesn't comply with project constitution"
    echo "Run: spec-kit-mcp analyze for details"
    exit 1
fi

echo "✅ Spec-kit validation passed"
```

### Integration with Project Management

Create integration with GitHub Issues:

```yaml
# .github/workflows/spec-from-issue.yml
name: Create Spec from Issue

on:
  issues:
    types: [labeled]

jobs:
  create-spec:
    if: github.event.label.name == 'needs-spec'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install spec-kit-mcp
        run: cargo install spec-kit-mcp

      - name: Generate specification from issue
        run: |
          spec-kit-mcp specify \
            --from-issue ${{ github.event.issue.number }} \
            --output .speckit/specs/issue-${{ github.event.issue.number }}.md

      - name: Create PR with specification
        run: |
          git checkout -b spec/issue-${{ github.event.issue.number }}
          git add .speckit/specs/
          git commit -m "spec: Add specification for issue #${{ github.event.issue.number }}"
          git push origin spec/issue-${{ github.event.issue.number }}
          gh pr create --title "Spec: Issue #${{ github.event.issue.number }}" \
            --body "Auto-generated specification from issue #${{ github.event.issue.number }}"
```

### Continuous Quality Monitoring

Create a dashboard that tracks:
- Specification coverage (% of features with specs)
- Constitution compliance score
- Implementation completeness
- Technical debt trends

Example quality tracking script:

```bash
#!/bin/bash
# scripts/quality-dashboard.sh

echo "# Spec-Kit Quality Dashboard"
echo ""
echo "Generated: $(date)"
echo ""

# Count specifications
spec_count=$(find .speckit/specs -name "*.md" | wc -l)
echo "📋 Specifications: $spec_count"

# Count implemented features
impl_count=$(find src -name "*.rs" -o -name "*.ts" | wc -l)
echo "💻 Implementation files: $impl_count"

# Check test coverage
coverage=$(cargo tarpaulin --out Stdout | grep "^%" | cut -d' ' -f1)
echo "🧪 Test coverage: $coverage"

# Analyze code quality
spec-kit-mcp analyze --json > quality.json
quality_score=$(jq '.quality_score' quality.json)
echo "⭐ Quality score: $quality_score/100"

# Check constitution compliance
compliance=$(jq '.constitution_compliance' quality.json)
echo "📜 Constitution compliance: $compliance%"

# Technical debt
debt_hours=$(jq '.technical_debt_hours' quality.json)
echo "⚠️  Technical debt: $debt_hours hours"
```

---

## Best Practices Summary

### Specification Best Practices

1. **Start with Why**: Always explain the problem before the solution
2. **Be Specific**: Include concrete acceptance criteria
3. **Think About Edge Cases**: Document error handling and edge cases
4. **Keep It Updated**: Specifications are living documents
5. **Review Together**: Get team input before implementation

### Implementation Best Practices

1. **Follow the Plan**: Stick to the generated plan unless you have good reason to deviate
2. **Test as You Go**: Write tests alongside implementation
3. **Commit Often**: Small, focused commits with clear messages
4. **Document Changes**: Update documentation when implementation differs from spec
5. **Verify Completeness**: Use checklists to ensure nothing is missed

### Team Collaboration Best Practices

1. **Share Context**: Make specifications accessible to all team members
2. **Review Specs First**: Review and approve specs before starting implementation
3. **Use Templates**: Standardize specification format across the team
4. **Track Progress**: Use task lists to show progress
5. **Learn and Improve**: Update the constitution based on lessons learned

### Quality Best Practices

1. **Analyze Regularly**: Run quality checks frequently
2. **Fix Early**: Address quality issues as soon as they're identified
3. **Automate Checks**: Use CI/CD to enforce quality standards
4. **Measure Trends**: Track quality metrics over time
5. **Continuous Improvement**: Regularly update standards and practices

---

## Troubleshooting

### Common Issues

**Issue**: Specifications are too vague

**Solution**: Use `speckit_clarify` to identify and address ambiguities before implementation

---

**Issue**: Implementation diverged from specification

**Solution**:
1. Update specification to match reality
2. Document why the change was necessary
3. Review the change with team
4. Update related documentation

---

**Issue**: Too many specifications, hard to find things

**Solution**:
1. Organize specs by feature area in subdirectories
2. Create an index file listing all specs
3. Use consistent naming conventions
4. Tag specs with metadata (priority, status, etc.)

---

**Issue**: Team not following spec-kit process

**Solution**:
1. Provide training and examples
2. Integrate into code review process
3. Use CI/CD to enforce compliance
4. Make it easy with templates and automation

---

## Next Steps

After completing these tutorials, you should:

1. **Customize for Your Team**: Adapt the spec-kit workflow to your team's needs
2. **Create Templates**: Build specification templates for common feature types
3. **Automate**: Integrate spec-kit into your CI/CD pipeline
4. **Share Knowledge**: Train team members on the spec-kit methodology
5. **Iterate**: Continuously improve your process based on experience

## Additional Resources

- [Spec-Kit MCP Documentation](https://github.com/lsendel/spec-kit-mcp)
- [GitHub Spec-Kit Methodology](https://github.com/github/spec-kit)
- [MCP Protocol Documentation](https://modelcontextprotocol.io/)
- [Examples Repository](./examples/)

---

**Happy spec-driven development!** 🚀
