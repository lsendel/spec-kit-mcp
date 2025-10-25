//! MCP Tools Implementation
//!
//! This module provides all the spec-kit tools exposed via MCP.

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

use crate::mcp::types::{ToolDefinition, ToolResult};
use crate::speckit::SpecKitCli;

pub mod analyze;
pub mod check;
pub mod checklist;
pub mod clarify;
pub mod constitution;
pub mod implement;
pub mod init;
pub mod plan;
pub mod specify;
pub mod tasks;

pub use analyze::AnalyzeTool;
pub use check::CheckTool;
pub use checklist::ChecklistTool;
pub use clarify::ClarifyTool;
pub use constitution::ConstitutionTool;
pub use implement::ImplementTool;
pub use init::InitTool;
pub use plan::PlanTool;
pub use specify::SpecifyTool;
pub use tasks::TasksTool;

/// Trait for all MCP tools
#[async_trait]
pub trait Tool: Send + Sync {
    /// Get the tool definition
    fn definition(&self) -> ToolDefinition;

    /// Execute the tool
    async fn execute(&self, params: Value) -> Result<ToolResult>;

    /// Get the tool name
    fn name(&self) -> String {
        self.definition().name.clone()
    }
}

/// Tool registry
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    /// Create a new tool registry
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Register a tool
    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        let name = tool.name();
        self.tools.insert(name, tool);
    }

    /// Get a tool by name
    pub fn get(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name).cloned()
    }

    /// List all tool definitions
    pub fn list_tools(&self) -> Vec<ToolDefinition> {
        self.tools.values().map(|tool| tool.definition()).collect()
    }

    /// Check if a tool exists
    pub fn has_tool(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }

    /// Get the number of registered tools
    pub fn len(&self) -> usize {
        self.tools.len()
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Create and populate the default tool registry
pub fn create_registry(cli: SpecKitCli) -> ToolRegistry {
    let mut registry = ToolRegistry::new();

    // Register all tools
    registry.register(Arc::new(InitTool::new(cli.clone())));
    registry.register(Arc::new(CheckTool::new(cli.clone())));
    registry.register(Arc::new(ConstitutionTool::new(cli.clone())));
    registry.register(Arc::new(SpecifyTool::new(cli.clone())));
    registry.register(Arc::new(PlanTool::new(cli.clone())));
    registry.register(Arc::new(TasksTool::new(cli.clone())));
    registry.register(Arc::new(ImplementTool::new(cli.clone())));
    registry.register(Arc::new(ClarifyTool::new(cli.clone())));
    registry.register(Arc::new(AnalyzeTool::new(cli.clone())));
    registry.register(Arc::new(ChecklistTool::new(cli)));

    tracing::info!(tool_count = registry.len(), "Tool registry created");

    registry
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = ToolRegistry::new();
        assert_eq!(registry.len(), 0);
        assert!(registry.is_empty());
    }

    #[test]
    fn test_registry_with_tools() {
        let cli = SpecKitCli::new();
        let registry = create_registry(cli);

        assert!(!registry.is_empty());
        assert!(registry.has_tool("speckit_init"));
        assert!(registry.has_tool("speckit_constitution"));
        assert!(registry.has_tool("speckit_specify"));
    }

    #[test]
    fn test_list_tools() {
        let cli = SpecKitCli::new();
        let registry = create_registry(cli);

        let tools = registry.list_tools();
        assert!(!tools.is_empty());

        // Verify all tools have proper definitions
        for tool in tools {
            assert!(!tool.name.is_empty());
            assert!(!tool.description.is_empty());
        }
    }
}
