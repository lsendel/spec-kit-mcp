//! Spec-Kit Implement Tool
//!
//! Executes implementation according to the task list.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;

use crate::mcp::types::{ContentBlock, ToolDefinition, ToolResult};
use crate::speckit::SpecKitCli;
use crate::tools::Tool;

/// Parameters for the speckit_implement tool
#[derive(Debug, Deserialize, Serialize)]
pub struct ImplementParams {
    /// Path to tasks file
    task_file: PathBuf,

    /// Additional context for implementation
    #[serde(default)]
    context: Option<String>,

    /// Output directory for implementation
    #[serde(default = "default_output_dir")]
    output_dir: PathBuf,
}

fn default_output_dir() -> PathBuf {
    PathBuf::from("./src")
}

/// Tool for executing implementation
pub struct ImplementTool {
    #[allow(dead_code)]
    cli: SpecKitCli,
}

impl ImplementTool {
    /// Create a new implement tool
    pub fn new(cli: SpecKitCli) -> Self {
        Self { cli }
    }
}

#[async_trait]
impl Tool for ImplementTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "speckit_implement".to_string(),
            description: "Execute implementation according to the task list, generating code and documentation".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "task_file": {
                        "type": "string",
                        "description": "Path to the tasks file (speckit.tasks)"
                    },
                    "context": {
                        "type": "string",
                        "description": "Additional context for implementation (e.g., existing code patterns, constraints)"
                    },
                    "output_dir": {
                        "type": "string",
                        "description": "Directory where code will be generated",
                        "default": "./src"
                    }
                },
                "required": ["task_file"]
            })
        }
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let params: ImplementParams =
            serde_json::from_value(params).context("Failed to parse implement parameters")?;

        tracing::info!(
            task_file = %params.task_file.display(),
            output_dir = %params.output_dir.display(),
            "Executing implementation"
        );

        // Read the tasks file
        let tasks_content = tokio::fs::read_to_string(&params.task_file)
            .await
            .context("Failed to read tasks file")?;

        // For now, we return guidance since actual implementation
        // requires AI-generated code based on specs
        let message = format!(
            "Implementation guidance based on {}\n\n\
            The task file contains the following items:\n\
            {}\n\n\
            Implementation approach:\n\
            1. Review each task in the task list\n\
            2. Implement tasks in order, respecting dependencies\n\
            3. Write tests alongside implementation\n\
            4. Update documentation as you go\n\
            5. Commit after completing each logical unit\n\n\
            Context: {}\n\n\
            Output directory: {}\n\n\
            Next step: Begin implementing the first task",
            params.task_file.display(),
            tasks_content
                .lines()
                .take(10)
                .collect::<Vec<_>>()
                .join("\n"),
            params.context.as_deref().unwrap_or("None provided"),
            params.output_dir.display()
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
    use tokio::fs;

    #[tokio::test]
    async fn test_implement_tool_definition() {
        let cli = SpecKitCli::new();
        let tool = ImplementTool::new(cli);
        let def = tool.definition();

        assert_eq!(def.name, "speckit_implement");
        assert!(!def.description.is_empty());
    }

    #[tokio::test]
    async fn test_implement_tool_execute() {
        let cli = SpecKitCli::new_test_mode();
        let tool = ImplementTool::new(cli);

        let dir = tempdir().unwrap();
        let task_file = dir.path().join("tasks.md");

        // Create dummy task file
        fs::write(&task_file, "Task 1: Implement feature\nTask 2: Write tests")
            .await
            .unwrap();

        let params = json!({
            "task_file": task_file.to_str().unwrap(),
            "context": "Using Rust 2021 edition",
            "output_dir": dir.path().join("src").to_str().unwrap()
        });

        let result = tool.execute(params).await.unwrap();
        assert!(result.is_error.is_none() || !result.is_error.unwrap());
    }
}
