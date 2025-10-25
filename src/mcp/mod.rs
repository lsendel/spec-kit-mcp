//! MCP (Model Context Protocol) Implementation
//!
//! This module provides the core MCP protocol implementation for the spec-kit MCP server.

pub mod protocol;
pub mod server;
pub mod transport;
pub mod types;

pub use protocol::*;
pub use server::*;
pub use transport::*;
pub use types::*;
