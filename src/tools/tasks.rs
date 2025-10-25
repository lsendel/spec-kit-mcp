//! Spec-Kit Tasks Tool
//!
//! Generates actionable task lists from technical plans.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;

use crate::mcp::types::{ContentBlock, ToolDefinition, ToolResult};
use crate::speckit::SpecKitCli;
use crate::tools::Tool;

/// Parameters for the speckit_tasks tool
#[derive(Debug, Deserialize, Serialize)]
pub struct TasksParams {
    /// Path to plan file
    plan_file: PathBuf,

    /// Breakdown level
    #[serde(default = "default_breakdown_level")]
    breakdown_level: String,

    /// Output path for tasks file
    #[serde(default = "default_tasks_path")]
    output_path: PathBuf,
}

fn default_breakdown_level() -> String {
    "medium".to_string()
}

fn default_tasks_path() -> PathBuf {
    PathBuf::from("./speckit.tasks")
}

/// Tool for generating task lists
pub struct TasksTool {
    cli: SpecKitCli,
}

impl TasksTool {
    /// Create a new tasks tool
    pub fn new(cli: SpecKitCli) -> Self {
        Self { cli }
    }
}

#[async_trait]
impl Tool for TasksTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "speckit_tasks".to_string(),
            description: "Generate actionable task lists from the technical plan, breaking down work into manageable items".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "plan_file": {
                        "type": "string",
                        "description": "Path to the plan file (speckit.plan)"
                    },
                    "breakdown_level": {
                        "type": "string",
                        "enum": ["high", "medium", "detailed"],
                        "default": "medium",
                        "description": "Level of task breakdown (high=major milestones, detailed=granular tasks)"
                    },
                    "output_path": {
                        "type": "string",
                        "description": "Path where the tasks file will be written",
                        "default": "./speckit.tasks"
                    }
                },
                "required": ["plan_file"]
            })
        }
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let params: TasksParams =
            serde_json::from_value(params).context("Failed to parse tasks parameters")?;

        tracing::info!(
            plan_file = %params.plan_file.display(),
            breakdown_level = %params.breakdown_level,
            output_path = %params.output_path.display(),
            "Generating task list"
        );

        // Execute spec-kit tasks command
        let result = self
            .cli
            .tasks(&params.plan_file, &params.output_path)
            .await?;

        if !result.is_success() {
            return Ok(ToolResult {
                content: vec![ContentBlock::text(format!(
                    "Failed to generate tasks: {}",
                    result.stderr
                ))],
                is_error: Some(true),
            });
        }

        let message = format!(
            "Task list generated successfully at {}\n\n\
            The task list includes:\n\
            - Prioritized actionable items\n\
            - Clear acceptance criteria\n\
            - Dependencies between tasks\n\
            - Estimated effort levels\n\n\
            Next step: Use speckit_implement tool to execute the tasks",
            params.output_path.display()
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
    async fn test_tasks_tool_definition() {
        let cli = SpecKitCli::new();
        let tool = TasksTool::new(cli);
        let def = tool.definition();

        assert_eq!(def.name, "speckit_tasks");
        assert!(!def.description.is_empty());
    }

    #[tokio::test]
    async fn test_tasks_tool_execute() {
        let cli = SpecKitCli::new_test_mode();
        let tool = TasksTool::new(cli);

        let dir = tempdir().unwrap();
        let plan_file = dir.path().join("plan.md");
        let output_path = dir.path().join("tasks.md");

        // Create dummy plan file
        fs::write(&plan_file, "Test plan").await.unwrap();

        let params = json!({
            "plan_file": plan_file.to_str().unwrap(),
            "breakdown_level": "medium",
            "output_path": output_path.to_str().unwrap()
        });

        let result = tool.execute(params).await.unwrap();
        assert!(result.is_error.is_none() || !result.is_error.unwrap());
    }
}
