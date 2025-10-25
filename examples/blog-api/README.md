# Blog API - Complete Spec-Kit Example

A production-ready REST API for a blog platform, built entirely using spec-driven development.

## Overview

This example demonstrates a complete spec-kit workflow for building a real-world REST API with:
- ✅ JWT Authentication
- ✅ User Management
- ✅ Post CRUD Operations
- ✅ Comments System
- ✅ Database Integration (PostgreSQL)
- ✅ API Documentation (OpenAPI/Swagger)
- ✅ Comprehensive Testing
- ✅ CI/CD Pipeline

## What You'll Build

```
POST   /api/v1/auth/register    - Register new user
POST   /api/v1/auth/login       - Login user
GET    /api/v1/auth/me          - Get current user

GET    /api/v1/posts            - List all posts
POST   /api/v1/posts            - Create new post (auth)
GET    /api/v1/posts/:id        - Get post by ID
PUT    /api/v1/posts/:id        - Update post (auth, owner)
DELETE /api/v1/posts/:id        - Delete post (auth, owner)

GET    /api/v1/posts/:id/comments    - List post comments
POST   /api/v1/posts/:id/comments    - Create comment (auth)
DELETE /api/v1/comments/:id          - Delete comment (auth, owner)

GET    /api/v1/users/:id        - Get user profile
PUT    /api/v1/users/:id        - Update user profile (auth, owner)
```

## Project Structure

```
blog-api/
├── .speckit/
│   ├── CONSTITUTION.md
│   ├── specs/
│   │   ├── 01-authentication.md
│   │   ├── 02-user-management.md
│   │   ├── 03-post-management.md
│   │   └── 04-comments.md
│   ├── plans/
│   │   ├── authentication-plan.md
│   │   ├── posts-plan.md
│   │   └── comments-plan.md
│   └── tasks/
│       ├── auth-tasks.md
│       ├── posts-tasks.md
│       └── comments-tasks.md
├── src/
│   ├── main.rs (or index.ts)
│   ├── auth/
│   │   ├── jwt.rs
│   │   ├── middleware.rs
│   │   └── handlers.rs
│   ├── models/
│   │   ├── user.rs
│   │   ├── post.rs
│   │   └── comment.rs
│   ├── routes/
│   ├── db/
│   └── errors/
├── tests/
│   ├── unit/
│   ├── integration/
│   └── api/
├── docs/
│   └── openapi.yaml
└── README.md
```

## Complete Workflow Walkthrough

### Phase 1: Project Setup

#### Step 1: Initialize Project

**Prompt your AI assistant**:
```
Use speckit_init to create a new project:
- Name: blog-api
- Type: REST API
- Language: Rust (or TypeScript/Python)
- Description: RESTful blog API with authentication and content management
```

**Generated**:
- `.speckit/CONSTITUTION.md` with API-specific principles
- Project structure

#### Step 2: Define Project Constitution

**Review and customize** `.speckit/CONSTITUTION.md`:

```markdown
# Blog API Constitution

## Core Purpose
A secure, scalable REST API for blog content management

## Technical Principles

### API Design
- RESTful architecture following HTTP semantics
- Versioned API (v1)
- JSON request/response format
- Consistent error handling
- Pagination for list endpoints

### Security
- JWT-based authentication
- Password hashing with bcrypt
- Input validation and sanitization
- Rate limiting
- CORS configuration

### Data Management
- PostgreSQL database
- Database migrations
- Connection pooling
- Transaction support

### Testing
- Unit tests for business logic
- Integration tests for database operations
- API tests for endpoints
- >80% code coverage

### Documentation
- OpenAPI/Swagger specification
- Inline code documentation
- README with setup instructions
- API usage examples
```

### Phase 2: Specification Creation

#### Step 3: Create Authentication Specification

**Prompt**:
```
Use speckit_specify to create a comprehensive specification for:
- Feature: "Authentication System"
- Include: User registration, login, JWT tokens, password requirements, error handling
```

**Generated**: `.speckit/specs/01-authentication.md`

<details>
<summary>Click to see complete Authentication Specification</summary>

```markdown
# Feature: Authentication System

## User Stories

### Registration
As a new user, I want to register an account so that I can create and manage blog posts.

### Login
As a registered user, I want to log in to access protected features.

### Token Management
As an API client, I want to use JWT tokens to authenticate requests.

## Acceptance Criteria

### Registration Endpoint
- [ ] POST /api/v1/auth/register accepts username, email, and password
- [ ] Usernames must be 3-30 characters, alphanumeric with underscores
- [ ] Emails must be valid format and unique
- [ ] Passwords must be minimum 8 characters with at least one uppercase, lowercase, number
- [ ] Passwords are hashed with bcrypt before storage
- [ ] Returns 201 with user object and JWT token on success
- [ ] Returns 400 for validation errors
- [ ] Returns 409 if email already exists

### Login Endpoint
- [ ] POST /api/v1/auth/login accepts email and password
- [ ] Validates credentials against database
- [ ] Returns 200 with JWT token on success
- [ ] Returns 401 for invalid credentials
- [ ] Rate limits to 5 attempts per 15 minutes per IP

### Token Validation
- [ ] JWT tokens contain user ID and expiration
- [ ] Tokens expire after 24 hours
- [ ] Protected endpoints verify token in Authorization header
- [ ] Returns 401 for missing or invalid tokens

## API Endpoints

### POST /api/v1/auth/register

**Request**:
\`\`\`json
{
  "username": "johndoe",
  "email": "john@example.com",
  "password": "SecurePass123"
}
\`\`\`

**Success Response** (201):
\`\`\`json
{
  "user": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "username": "johndoe",
    "email": "john@example.com",
    "created_at": "2024-01-15T10:30:00Z"
  },
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
\`\`\`

**Error Response** (400):
\`\`\`json
{
  "error": "Validation failed",
  "details": {
    "password": "Password must contain at least one uppercase letter"
  }
}
\`\`\`

### POST /api/v1/auth/login

**Request**:
\`\`\`json
{
  "email": "john@example.com",
  "password": "SecurePass123"
}
\`\`\`

**Success Response** (200):
\`\`\`json
{
  "user": {
    "id": "123e4567-e89b-12d3-a456-426614174000",
    "username": "johndoe",
    "email": "john@example.com"
  },
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
\`\`\`

**Error Response** (401):
\`\`\`json
{
  "error": "Invalid credentials"
}
\`\`\`

### GET /api/v1/auth/me

**Headers**:
\`\`\`
Authorization: Bearer {token}
\`\`\`

**Success Response** (200):
\`\`\`json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "username": "johndoe",
  "email": "john@example.com",
  "created_at": "2024-01-15T10:30:00Z"
}
\`\`\`

## Data Models

### User
\`\`\`rust
struct User {
    id: Uuid,
    username: String,       // 3-30 chars, unique
    email: String,          // valid email, unique
    password_hash: String,  // bcrypt hash
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
\`\`\`

### JWT Claims
\`\`\`rust
struct Claims {
    sub: Uuid,              // user ID
    exp: i64,               // expiration timestamp
}
\`\`\`

## Security Considerations

1. **Password Storage**: Never store plain passwords; always use bcrypt
2. **Token Security**: Use secure random secret for JWT signing
3. **HTTPS Only**: Tokens should only be transmitted over HTTPS
4. **Rate Limiting**: Implement rate limiting on auth endpoints
5. **Input Validation**: Sanitize all user inputs

## Testing Strategy

### Unit Tests
- Password validation logic
- Username validation
- Email validation
- Token generation and verification
- Password hashing

### Integration Tests
- User registration flow
- Login flow with database
- Token validation middleware

### API Tests
- Register with valid data
- Register with invalid data (various cases)
- Login with correct credentials
- Login with wrong password
- Access protected endpoint with/without token
- Token expiration handling

## Implementation Notes

1. Use `jsonwebtoken` crate for JWT (Rust) or `jsonwebtoken` npm package (Node.js)
2. Use `bcrypt` for password hashing
3. Store JWT secret in environment variable
4. Implement middleware for token validation
5. Use database transactions for user creation

## Open Questions

- [ ] Should we implement refresh tokens?
- [ ] Do we need email verification?
- [ ] Should we support OAuth (Google, GitHub)?
- [ ] Password reset functionality needed?
```

</details>

#### Step 4: Create Post Management Specification

**Prompt**:
```
Use speckit_specify to create specification for:
- Feature: "Post Management"
- Include: CRUD operations, authentication requirements, authorization rules
```

**Generated**: `.speckit/specs/03-post-management.md`

#### Step 5: Create Comments Specification

**Prompt**:
```
Use speckit_specify to create specification for:
- Feature: "Comments System"
- Include: Comment creation, nested comments support, moderation
```

**Generated**: `.speckit/specs/04-comments.md`

### Phase 3: Planning

#### Step 6: Create Implementation Plans

**Prompt**:
```
Use speckit_plan to create implementation plans for each specification:
1. Authentication (foundation - must be first)
2. User Management
3. Post Management (depends on auth)
4. Comments (depends on posts and auth)

For each plan, include:
- Implementation order
- Dependencies
- Database schema
- Testing approach
```

**Generated**:
- `.speckit/plans/authentication-plan.md`
- `.speckit/plans/posts-plan.md`
- `.speckit/plans/comments-plan.md`

<details>
<summary>Authentication Implementation Plan Example</summary>

```markdown
# Authentication Implementation Plan

## Overview
Implement JWT-based authentication system as the foundation for the blog API.

## Dependencies
- Database setup (PostgreSQL)
- Basic API server structure
- Error handling utilities

## Implementation Order

### 1. Database Setup (Priority: Critical)
**Tasks**:
- Set up PostgreSQL connection
- Create users table migration
- Implement connection pooling
- Add database error handling

**Schema**:
\`\`\`sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(30) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
\`\`\`

### 2. User Model (Priority: Critical)
**Tasks**:
- Create User struct/model
- Implement validation functions
- Add password hashing utilities
- Create database queries (insert, find by email, find by ID)

**Files**:
- `src/models/user.rs`
- `src/db/users.rs`

### 3. JWT Utilities (Priority: Critical)
**Tasks**:
- Implement token generation
- Implement token verification
- Create Claims struct
- Add token middleware

**Files**:
- `src/auth/jwt.rs`
- `src/auth/middleware.rs`

### 4. Registration Endpoint (Priority: High)
**Tasks**:
- Create registration handler
- Implement input validation
- Handle duplicate email/username
- Return user and token

**Files**:
- `src/auth/handlers.rs`
- `src/routes/auth.rs`

### 5. Login Endpoint (Priority: High)
**Tasks**:
- Create login handler
- Verify credentials
- Generate and return token
- Implement rate limiting

### 6. Protected Endpoint Middleware (Priority: High)
**Tasks**:
- Extract token from headers
- Verify token validity
- Load user from database
- Attach user to request context

### 7. Testing (Priority: High)
**Tasks**:
- Unit tests for validation
- Unit tests for JWT functions
- Integration tests for auth flow
- API tests for all endpoints

## Testing Strategy

### Unit Tests
\`\`\`rust
#[test]
fn test_password_validation() {
    assert!(validate_password("SecurePass123").is_ok());
    assert!(validate_password("weak").is_err());
}

#[test]
fn test_jwt_generation_and_verification() {
    let user_id = Uuid::new_v4();
    let token = generate_token(user_id).unwrap();
    let claims = verify_token(&token).unwrap();
    assert_eq!(claims.sub, user_id);
}
\`\`\`

### Integration Tests
\`\`\`rust
#[tokio::test]
async fn test_user_registration_flow() {
    let pool = setup_test_db().await;
    let user_data = RegisterRequest {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "SecurePass123".to_string(),
    };

    let result = register_user(&pool, user_data).await;
    assert!(result.is_ok());

    let user = result.unwrap();
    assert_eq!(user.username, "testuser");
}
\`\`\`

### API Tests
\`\`\`bash
# Test registration
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"testuser","email":"test@example.com","password":"SecurePass123"}'

# Should return 201 with user and token
\`\`\`

## Estimated Effort
- Database setup: 2 hours
- User model: 3 hours
- JWT utilities: 2 hours
- Registration endpoint: 2 hours
- Login endpoint: 2 hours
- Middleware: 2 hours
- Testing: 4 hours
**Total: ~17 hours**

## Risks and Mitigations

**Risk**: Token secret leaked
**Mitigation**: Use environment variables, never commit secrets

**Risk**: SQL injection
**Mitigation**: Use parameterized queries/ORM

**Risk**: Weak passwords
**Mitigation**: Enforce password requirements

## Success Criteria
- [ ] All unit tests passing
- [ ] All integration tests passing
- [ ] All API tests passing
- [ ] >80% code coverage
- [ ] Security audit passed
- [ ] Documentation complete
```

</details>

### Phase 4: Task Generation

#### Step 7: Generate Task Checklists

**Prompt**:
```
Use speckit_tasks to generate detailed task checklists from each plan:
1. .speckit/plans/authentication-plan.md → .speckit/tasks/auth-tasks.md
2. .speckit/plans/posts-plan.md → .speckit/tasks/posts-tasks.md
3. .speckit/plans/comments-plan.md → .speckit/tasks/comments-tasks.md
```

### Phase 5: Implementation

#### Step 8: Implement Authentication

**Prompt**:
```
Use speckit_implement with:
- Task file: .speckit/tasks/auth-tasks.md
- Context: Using Rust with Axum framework, PostgreSQL database, JWT tokens
- Output directory: ./src

Implement the authentication system following the specification and plan.
```

**What gets implemented**:
- Database schema and migrations
- User model with validation
- JWT token generation/verification
- Registration and login endpoints
- Authentication middleware
- Comprehensive tests

#### Step 9: Implement Posts

**Prompt**:
```
Use speckit_implement with:
- Task file: .speckit/tasks/posts-tasks.md
- Context: Build on existing auth system, require authentication for create/update/delete
- Output directory: ./src

Implement the post management system.
```

#### Step 10: Implement Comments

**Prompt**:
```
Use speckit_implement with:
- Task file: .speckit/tasks/comments-tasks.md
- Context: Build on posts and auth, support nested comments
- Output directory: ./src

Implement the comments system.
```

### Phase 6: Testing and Quality

#### Step 11: Run Tests

```bash
# Run all tests
cargo test  # or npm test, or pytest

# Run with coverage
cargo tarpaulin --out Html

# Check coverage report
open tarpaulin-report.html
```

#### Step 12: Quality Analysis

**Prompt**:
```
Use speckit_analyze on the entire codebase to check:
- API consistency
- Error handling completeness
- Security best practices
- Test coverage
- Constitution compliance
```

#### Step 13: Create Review Checklist

**Prompt**:
```
Use speckit_checklist to create a comprehensive review checklist covering:
- All acceptance criteria met
- All tests passing
- Security requirements satisfied
- Documentation complete
- Performance benchmarks met
```

### Phase 7: Documentation

#### Step 14: Generate API Documentation

Create OpenAPI/Swagger documentation:

**Prompt**:
```
Based on our implemented API, generate complete OpenAPI 3.0 specification with:
- All endpoints documented
- Request/response schemas
- Authentication requirements
- Error responses
- Examples
```

**Generated**: `docs/openapi.yaml`

### Phase 8: Deployment

#### Step 15: Create Deployment Plan

**Prompt**:
```
Use speckit_specify to create a deployment specification:
- Infrastructure requirements (Docker, Kubernetes, or cloud service)
- Environment configuration
- Database migrations strategy
- Monitoring and logging
- CI/CD pipeline
```

## Running the Complete Example

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install PostgreSQL
# macOS: brew install postgresql
# Ubuntu: sudo apt-get install postgresql

# Install spec-kit-mcp
cargo install spec-kit-mcp
```

### Setup

```bash
# Clone or navigate to the example
cd examples/blog-api

# Set up database
createdb blog_api_dev
psql blog_api_dev < migrations/001_initial.sql

# Configure environment
cp .env.example .env
# Edit .env with your database credentials and JWT secret

# Build
cargo build
```

### Run the API

```bash
# Start the server
cargo run

# Server will start on http://localhost:8080
```

### Test the API

```bash
# Register a user
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","email":"alice@example.com","password":"SecurePass123"}'

# Response:
# {
#   "user": {"id": "...", "username": "alice", ...},
#   "token": "eyJhbGc..."
# }

# Save the token for authenticated requests
TOKEN="eyJhbGc..."

# Create a post
curl -X POST http://localhost:8080/api/v1/posts \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"title":"My First Post","body":"Hello, world!","published":true}'

# List posts
curl http://localhost:8080/api/v1/posts

# Add a comment
curl -X POST http://localhost:8080/api/v1/posts/1/comments \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{"body":"Great post!"}'
```

## Learning Outcomes

After completing this example, you will have learned:

✅ **Specification-Driven API Development**
- Writing comprehensive API specifications
- Defining data models and schemas
- Documenting security requirements

✅ **Systematic Implementation**
- Breaking down features into phases
- Managing dependencies between features
- Following implementation plans

✅ **Security Best Practices**
- JWT authentication implementation
- Password hashing and validation
- Authorization and access control

✅ **Database Design**
- Schema design for relational data
- Migrations management
- Query optimization

✅ **Testing Strategies**
- Unit testing business logic
- Integration testing database operations
- API testing endpoints

✅ **Quality Assurance**
- Code quality metrics
- Security auditing
- Performance optimization

## Extensions

Want to add more features? Try:

1. **Pagination**: Add cursor-based pagination for list endpoints
2. **Search**: Full-text search for posts
3. **Tags**: Tagging system for posts
4. **Likes**: Like system for posts and comments
5. **Followers**: User following system
6. **Media**: Image upload for posts
7. **Notifications**: Real-time notifications

For each, follow the spec-kit workflow:
1. Specify → 2. Plan → 3. Tasks → 4. Implement → 5. Test → 6. Analyze

## Next Steps

- ✅ Complete Blog API example
- → Try [Web App](../web-app/) for full-stack
- → Challenge yourself with [Refactoring](../refactoring/)

## Resources

- [OpenAPI Specification](./docs/openapi.yaml)
- [Database Schema](./migrations/)
- [API Testing Collection](./tests/api/)
- [Deployment Guide](./docs/DEPLOYMENT.md)

---

**Questions?** Open an issue at [GitHub Issues](https://github.com/lsendel/spec-kit-mcp/issues)
