//! Spec-Kit Specify Tool
//!
//! Defines requirements and user stories.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;

use crate::mcp::types::{ContentBlock, ToolDefinition, ToolResult};
use crate::speckit::SpecKitCli;
use crate::tools::Tool;

/// Parameters for the speckit_specify tool
#[derive(Debug, Deserialize, Serialize)]
pub struct SpecifyParams {
    /// Requirements to specify
    requirements: String,

    /// User stories (optional)
    #[serde(default)]
    user_stories: Option<String>,

    /// Output path for specification file
    #[serde(default = "default_specify_path")]
    output_path: PathBuf,

    /// Output format
    #[serde(default = "default_format")]
    format: String,
}

fn default_specify_path() -> PathBuf {
    PathBuf::from("./speckit.specify")
}

fn default_format() -> String {
    "markdown".to_string()
}

/// Tool for creating specifications
pub struct SpecifyTool {
    cli: SpecKitCli,
}

impl SpecifyTool {
    /// Create a new specify tool
    pub fn new(cli: SpecKitCli) -> Self {
        Self { cli }
    }
}

#[async_trait]
impl Tool for SpecifyTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "speckit_specify".to_string(),
            description: "Define what you want to build - requirements, user stories, and acceptance criteria".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "requirements": {
                        "type": "string",
                        "description": "The requirements to specify. Can include features, constraints, user needs, etc."
                    },
                    "user_stories": {
                        "type": "string",
                        "description": "Optional user stories in 'As a... I want... So that...' format"
                    },
                    "output_path": {
                        "type": "string",
                        "description": "Path where the specification file will be written",
                        "default": "./speckit.specify"
                    },
                    "format": {
                        "type": "string",
                        "enum": ["markdown", "yaml", "json"],
                        "default": "markdown",
                        "description": "Output format for the specification"
                    }
                },
                "required": ["requirements"]
            })
        }
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let params: SpecifyParams =
            serde_json::from_value(params).context("Failed to parse specify parameters")?;

        tracing::info!(
            output_path = %params.output_path.display(),
            format = %params.format,
            "Creating specification"
        );

        // Format the specification content
        let mut content = format!(
            "# Specification\n\n## Requirements\n\n{}\n",
            params.requirements
        );

        if let Some(stories) = params.user_stories {
            content.push_str(&format!("\n## User Stories\n\n{}\n", stories));
        }

        // Write specification file
        let result = self
            .cli
            .specify(&content, &params.output_path, &params.format)
            .await?;

        if !result.is_success() {
            return Ok(ToolResult {
                content: vec![ContentBlock::text(format!(
                    "Failed to create specification: {}",
                    result.stderr
                ))],
                is_error: Some(true),
            });
        }

        let message = format!(
            "Specification created successfully at {}\n\n\
            The specification defines:\n\
            - What needs to be built (requirements)\n\
            - Who it's for and why (user stories)\n\
            - Success criteria (acceptance criteria)\n\n\
            Next step: Use speckit_plan tool to create a technical plan",
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
    async fn test_specify_tool_definition() {
        let cli = SpecKitCli::new();
        let tool = SpecifyTool::new(cli);
        let def = tool.definition();

        assert_eq!(def.name, "speckit_specify");
        assert!(!def.description.is_empty());
    }

    #[tokio::test]
    async fn test_specify_tool_execute() {
        let cli = SpecKitCli::new_test_mode();
        let tool = SpecifyTool::new(cli);

        let dir = tempdir().unwrap();
        let output_path = dir.path().join("specification.md");

        let params = json!({
            "requirements": "User authentication system with OAuth2 support",
            "user_stories": "As a user, I want to login with Google, so that I don't need another password",
            "output_path": output_path.to_str().unwrap()
        });

        let result = tool.execute(params).await.unwrap();
        assert!(result.is_error.is_none() || !result.is_error.unwrap());
        assert!(output_path.exists());
    }
}
