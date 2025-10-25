//! Spec-Kit MCP Server
//!
//! MCP server that enables AI coding assistants to use spec-driven development practices
//! via the GitHub Spec-Kit toolkit.
//!
//! # Features
//!
//! - **9 Spec-Kit Tools**: Complete workflow from constitution to implementation
//! - **MCP Protocol**: Full JSON-RPC 2.0 implementation
//! - **Async/Await**: Built on Tokio for high performance
//! - **Type Safety**: Comprehensive type system with validation
//! - **Error Handling**: Helpful error messages for debugging
//!
//! # Quick Start
//!
//! ## As a Binary
//!
//! ```bash
//! # Via cargo
//! cargo install spec-kit-mcp
//! spec-kit-mcp
//!
//! # Via npx
//! npx @speckit/mcp
//! ```
//!
//! ## Configure in Claude Code
//!
//! Add to `.mcp.json`:
//!
//! ```json
//! {
//!   "mcpServers": {
//!     "spec-kit": {
//!       "command": "npx",
//!       "args": ["@speckit/mcp@latest"]
//!     }
//!   }
//! }
//! ```
//!
//! # Architecture
//!
//! ```text
//! AI Agent → MCP Protocol → Tool Registry → Spec-Kit CLI → File System
//! ```
//!
//! # Available Tools
//!
//! 1. `speckit_init` - Initialize a new spec-kit project
//! 2. `speckit_constitution` - Create governing principles
//! 3. `speckit_specify` - Define requirements and user stories
//! 4. `speckit_plan` - Create technical implementation plan
//! 5. `speckit_tasks` - Generate actionable task list
//! 6. `speckit_implement` - Execute implementation
//! 7. `speckit_clarify` - Clarify ambiguous requirements
//! 8. `speckit_analyze` - Analyze cross-artifact consistency
//! 9. `speckit_checklist` - Generate validation checklist
//!
//! # Example Usage
//!
//! ```rust,no_run
//! use spec_kit_mcp::{McpServer, create_registry, SpecKitCli};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Create CLI interface
//!     let cli = SpecKitCli::new();
//!
//!     // Create tool registry
//!     let registry = create_registry(cli);
//!
//!     // Create and run server
//!     let mut server = McpServer::new(registry);
//!     server.run().await?;
//!
//!     Ok(())
//! }
//! ```

pub mod mcp;
pub mod speckit;
pub mod tools;
pub mod config;
pub mod utils;

// Re-export main types
pub use mcp::{McpServer, ProtocolHandler, StdioTransport};
pub use speckit::{SpecKitCli, SpecKitError};
pub use tools::{Tool, ToolRegistry, create_registry};
