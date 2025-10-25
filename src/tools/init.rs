//! Spec-Kit Init Tool
//!
//! Initializes a new spec-kit project.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;

use crate::mcp::types::{ContentBlock, ToolDefinition, ToolResult};
use crate::speckit::SpecKitCli;
use crate::tools::Tool;

/// Parameters for the speckit_init tool
#[derive(Debug, Deserialize, Serialize)]
pub struct InitParams {
    /// Project name
    project_name: String,

    /// Project path (defaults to current directory)
    #[serde(default = "default_project_path")]
    project_path: PathBuf,
}

fn default_project_path() -> PathBuf {
    PathBuf::from(".")
}

/// Tool for initializing spec-kit projects
pub struct InitTool {
    cli: SpecKitCli,
}

impl InitTool {
    /// Create a new init tool
    pub fn new(cli: SpecKitCli) -> Self {
        Self { cli }
    }
}

#[async_trait]
impl Tool for InitTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "speckit_init".to_string(),
            description: "Initialize a new spec-kit project with proper directory structure and configuration files".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "project_name": {
                        "type": "string",
                        "description": "Name of the project to initialize"
                    },
                    "project_path": {
                        "type": "string",
                        "description": "Path where the project should be created",
                        "default": "."
                    }
                },
                "required": ["project_name"]
            })
        }
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let params: InitParams =
            serde_json::from_value(params).context("Failed to parse init parameters")?;

        tracing::info!(
            project_name = %params.project_name,
            project_path = %params.project_path.display(),
            "Initializing spec-kit project"
        );

        // Execute spec-kit init command
        let result = self
            .cli
            .init(&params.project_name, &params.project_path)
            .await?;

        if !result.is_success() {
            return Ok(ToolResult {
                content: vec![ContentBlock::text(format!(
                    "Failed to initialize project: {}",
                    result.stderr
                ))],
                is_error: Some(true),
            });
        }

        let message = format!(
            "Successfully initialized spec-kit project '{}' at {}\n\n\
            Next steps:\n\
            1. Navigate to the project: cd {}\n\
            2. Create constitution: Use speckit_constitution tool\n\
            3. Define requirements: Use speckit_specify tool\n\
            4. Create technical plan: Use speckit_plan tool\n\
            5. Generate tasks: Use speckit_tasks tool\n\
            6. Implement: Use speckit_implement tool",
            params.project_name,
            params.project_path.display(),
            params.project_path.display()
        );

        Ok(ToolResult {
            content: vec![ContentBlock::text(message)],
            is_error: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_init_tool_definition() {
        let cli = SpecKitCli::new();
        let tool = InitTool::new(cli);
        let def = tool.definition();

        assert_eq!(def.name, "speckit_init");
        assert!(!def.description.is_empty());
    }

    #[tokio::test]
    async fn test_init_tool_execute() {
        let cli = SpecKitCli::new_test_mode();
        let tool = InitTool::new(cli);

        let dir = tempdir().unwrap();
        let params = json!({
            "project_name": "test-project",
            "project_path": dir.path().to_str().unwrap()
        });

        let result = tool.execute(params).await.unwrap();
        assert!(result.is_error.is_none() || !result.is_error.unwrap());
    }
}
