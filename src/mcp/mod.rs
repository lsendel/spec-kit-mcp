//! MCP (Model Context Protocol) Implementation
//!
//! This module provides the core MCP protocol implementation for the spec-kit MCP server.

pub mod types;
pub mod protocol;
pub mod server;
pub mod transport;

pub use types::*;
pub use protocol::*;
pub use server::*;
pub use transport::*;
