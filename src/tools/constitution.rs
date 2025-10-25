//! Spec-Kit Constitution Tool
//!
//! Creates project governing principles and development guidelines.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;

use crate::mcp::types::{ContentBlock, ToolDefinition, ToolResult};
use crate::speckit::SpecKitCli;
use crate::tools::Tool;

/// Parameters for the speckit_constitution tool
#[derive(Debug, Deserialize, Serialize)]
pub struct ConstitutionParams {
    /// Core principles and values
    principles: String,

    /// Technical constraints (optional)
    #[serde(default)]
    constraints: Option<String>,

    /// Output path for constitution file
    #[serde(default = "default_constitution_path")]
    output_path: PathBuf,
}

fn default_constitution_path() -> PathBuf {
    PathBuf::from("./speckit.constitution")
}

/// Tool for creating project constitutions
pub struct ConstitutionTool {
    cli: SpecKitCli,
}

impl ConstitutionTool {
    /// Create a new constitution tool
    pub fn new(cli: SpecKitCli) -> Self {
        Self { cli }
    }
}

#[async_trait]
impl Tool for ConstitutionTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "speckit_constitution".to_string(),
            description: "Create or update project governing principles, development standards, and technical constraints".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "principles": {
                        "type": "string",
                        "description": "Core principles and values that govern the project (e.g., simplicity, performance, security)"
                    },
                    "constraints": {
                        "type": "string",
                        "description": "Technical constraints and boundaries (optional)"
                    },
                    "output_path": {
                        "type": "string",
                        "description": "Path where the constitution file will be written",
                        "default": "./speckit.constitution"
                    }
                },
                "required": ["principles"]
            })
        }
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let params: ConstitutionParams = serde_json::from_value(params)
            .context("Failed to parse constitution parameters")?;

        tracing::info!(
            output_path = %params.output_path.display(),
            "Creating constitution"
        );

        // Format the constitution content
        let mut content = format!("# Project Constitution\n\n## Core Principles\n\n{}\n", params.principles);

        if let Some(constraints) = params.constraints {
            content.push_str(&format!("\n## Technical Constraints\n\n{}\n", constraints));
        }

        // Write constitution file
        let result = self
            .cli
            .constitution(&content, &params.output_path)
            .await?;

        if !result.is_success() {
            return Ok(ToolResult {
                content: vec![ContentBlock::text(format!(
                    "Failed to create constitution: {}",
                    result.stderr
                ))],
                is_error: Some(true),
            });
        }

        let message = format!(
            "Constitution created successfully at {}\n\n\
            The constitution defines:\n\
            - Core principles that guide development\n\
            - Technical constraints and boundaries\n\
            - Standards for code quality and architecture\n\n\
            Next step: Use speckit_specify tool to define requirements",
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

    #[tokio::test]
    async fn test_constitution_tool_definition() {
        let cli = SpecKitCli::new();
        let tool = ConstitutionTool::new(cli);
        let def = tool.definition();

        assert_eq!(def.name, "speckit_constitution");
        assert!(!def.description.is_empty());
    }

    #[tokio::test]
    async fn test_constitution_tool_execute() {
        let cli = SpecKitCli::new_test_mode();
        let tool = ConstitutionTool::new(cli);

        let dir = tempdir().unwrap();
        let output_path = dir.path().join("constitution.md");

        let params = json!({
            "principles": "Simplicity, Performance, Security",
            "constraints": "Must support Python 3.11+",
            "output_path": output_path.to_str().unwrap()
        });

        let result = tool.execute(params).await.unwrap();
        assert!(result.is_error.is_none() || !result.is_error.unwrap());
        assert!(output_path.exists());
    }
}
