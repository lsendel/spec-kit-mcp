//! Spec-Kit Plan Tool
//!
//! Creates technical implementation plans.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;

use crate::mcp::types::{ContentBlock, ToolDefinition, ToolResult};
use crate::speckit::SpecKitCli;
use crate::tools::Tool;

/// Parameters for the speckit_plan tool
#[derive(Debug, Deserialize, Serialize)]
pub struct PlanParams {
    /// Path to specification file
    spec_file: PathBuf,

    /// Tech stack to use
    #[serde(default)]
    tech_stack: Option<String>,

    /// Output path for plan file
    #[serde(default = "default_plan_path")]
    output_path: PathBuf,
}

fn default_plan_path() -> PathBuf {
    PathBuf::from("./speckit.plan")
}

/// Tool for creating technical plans
pub struct PlanTool {
    cli: SpecKitCli,
}

impl PlanTool {
    /// Create a new plan tool
    pub fn new(cli: SpecKitCli) -> Self {
        Self { cli }
    }
}

#[async_trait]
impl Tool for PlanTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "speckit_plan".to_string(),
            description: "Create a technical implementation plan based on the specification, including architecture, tech stack, and approach".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "spec_file": {
                        "type": "string",
                        "description": "Path to the specification file (speckit.specify)"
                    },
                    "tech_stack": {
                        "type": "string",
                        "description": "Technology stack to use (e.g., 'Rust + Tokio', 'Python + FastAPI')"
                    },
                    "output_path": {
                        "type": "string",
                        "description": "Path where the plan file will be written",
                        "default": "./speckit.plan"
                    }
                },
                "required": ["spec_file"]
            })
        }
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let params: PlanParams = serde_json::from_value(params)
            .context("Failed to parse plan parameters")?;

        tracing::info!(
            spec_file = %params.spec_file.display(),
            output_path = %params.output_path.display(),
            "Creating technical plan"
        );

        // Execute spec-kit plan command
        let result = self
            .cli
            .plan(&params.spec_file, &params.output_path)
            .await?;

        if !result.is_success() {
            return Ok(ToolResult {
                content: vec![ContentBlock::text(format!(
                    "Failed to create plan: {}",
                    result.stderr
                ))],
                is_error: Some(true),
            });
        }

        let message = format!(
            "Technical plan created successfully at {}\n\n\
            The plan includes:\n\
            - Architecture and system design\n\
            - Technology stack and frameworks\n\
            - Implementation approach\n\
            - Module breakdown\n\n\
            Next step: Use speckit_tasks tool to generate actionable tasks",
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
    async fn test_plan_tool_definition() {
        let cli = SpecKitCli::new();
        let tool = PlanTool::new(cli);
        let def = tool.definition();

        assert_eq!(def.name, "speckit_plan");
        assert!(!def.description.is_empty());
    }

    #[tokio::test]
    async fn test_plan_tool_execute() {
        let cli = SpecKitCli::new_test_mode();
        let tool = PlanTool::new(cli);

        let dir = tempdir().unwrap();
        let spec_file = dir.path().join("spec.md");
        let output_path = dir.path().join("plan.md");

        // Create dummy spec file
        fs::write(&spec_file, "Test specification").await.unwrap();

        let params = json!({
            "spec_file": spec_file.to_str().unwrap(),
            "tech_stack": "Rust + Tokio",
            "output_path": output_path.to_str().unwrap()
        });

        let result = tool.execute(params).await.unwrap();
        assert!(result.is_error.is_none() || !result.is_error.unwrap());
    }
}
