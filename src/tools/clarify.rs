//! Spec-Kit Clarify Tool
//!
//! Identifies and clarifies ambiguous requirements.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;

use crate::mcp::types::{ContentBlock, ToolDefinition, ToolResult};
use crate::speckit::SpecKitCli;
use crate::tools::Tool;

/// Parameters for the speckit_clarify tool
#[derive(Debug, Deserialize, Serialize)]
pub struct ClarifyParams {
    /// Path to specification file
    spec_file: PathBuf,

    /// Specific questions to clarify (optional)
    #[serde(default)]
    questions: Option<Vec<String>>,

    /// Output path for clarifications
    #[serde(default = "default_clarify_path")]
    output_path: PathBuf,
}

fn default_clarify_path() -> PathBuf {
    PathBuf::from("./speckit.clarify")
}

/// Tool for clarifying specifications
pub struct ClarifyTool {
    #[allow(dead_code)]
    cli: SpecKitCli,
}

impl ClarifyTool {
    /// Create a new clarify tool
    pub fn new(cli: SpecKitCli) -> Self {
        Self { cli }
    }
}

#[async_trait]
impl Tool for ClarifyTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "speckit_clarify".to_string(),
            description: "Identify underspecified areas in the specification and generate clarification questions".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "spec_file": {
                        "type": "string",
                        "description": "Path to the specification file to analyze"
                    },
                    "questions": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "Specific questions to address (optional - will auto-detect if not provided)"
                    },
                    "output_path": {
                        "type": "string",
                        "description": "Path where clarifications will be written",
                        "default": "./speckit.clarify"
                    }
                },
                "required": ["spec_file"]
            })
        }
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let params: ClarifyParams = serde_json::from_value(params)
            .context("Failed to parse clarify parameters")?;

        tracing::info!(
            spec_file = %params.spec_file.display(),
            "Analyzing specification for ambiguities"
        );

        // Read the specification
        let spec_content = tokio::fs::read_to_string(&params.spec_file)
            .await
            .context("Failed to read specification file")?;

        // Analyze for common ambiguities
        let mut clarifications = Vec::new();

        // Check for vague terms
        let vague_terms = ["maybe", "probably", "might", "could", "should consider"];
        for term in &vague_terms {
            if spec_content.to_lowercase().contains(term) {
                clarifications.push(format!(
                    "Found vague term '{}' - needs concrete definition",
                    term
                ));
            }
        }

        // Check for missing details
        if !spec_content.to_lowercase().contains("performance") {
            clarifications.push("Performance requirements not specified".to_string());
        }
        if !spec_content.to_lowercase().contains("error") {
            clarifications.push("Error handling approach not specified".to_string());
        }
        if !spec_content.to_lowercase().contains("test") {
            clarifications.push("Testing strategy not specified".to_string());
        }

        // Add user-provided questions
        if let Some(questions) = params.questions {
            clarifications.extend(questions.into_iter().map(|q| format!("Question: {}", q)));
        }

        // Create clarification document
        let mut content = String::from("# Specification Clarifications\n\n");
        content.push_str(&format!("Source: {}\n\n", params.spec_file.display()));
        content.push_str("## Issues Found\n\n");

        if clarifications.is_empty() {
            content.push_str("✓ No major ambiguities detected.\n");
            content.push_str("\nThe specification appears well-defined.\n");
        } else {
            for (i, clarification) in clarifications.iter().enumerate() {
                content.push_str(&format!("{}. {}\n", i + 1, clarification));
            }
            content.push_str("\n## Recommendations\n\n");
            content.push_str("1. Address each issue above\n");
            content.push_str("2. Update the specification with concrete details\n");
            content.push_str("3. Review with stakeholders\n");
            content.push_str("4. Re-run clarify to verify improvements\n");
        }

        // Write clarifications
        tokio::fs::write(&params.output_path, &content)
            .await
            .context("Failed to write clarifications")?;

        let message = format!(
            "Clarification analysis complete!\n\n\
            Analyzed: {}\n\
            Issues found: {}\n\
            Output: {}\n\n\
            {}",
            params.spec_file.display(),
            clarifications.len(),
            params.output_path.display(),
            if clarifications.is_empty() {
                "✓ Specification is well-defined"
            } else {
                "⚠ Please review and address the identified issues"
            }
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
    async fn test_clarify_tool_definition() {
        let cli = SpecKitCli::new();
        let tool = ClarifyTool::new(cli);
        let def = tool.definition();

        assert_eq!(def.name, "speckit_clarify");
        assert!(!def.description.is_empty());
    }

    #[tokio::test]
    async fn test_clarify_tool_execute() {
        let cli = SpecKitCli::new_test_mode();
        let tool = ClarifyTool::new(cli);

        let dir = tempdir().unwrap();
        let spec_file = dir.path().join("spec.md");
        let output_path = dir.path().join("clarify.md");

        // Create spec with ambiguities
        fs::write(&spec_file, "We might add OAuth. Performance should be good.").await.unwrap();

        let params = json!({
            "spec_file": spec_file.to_str().unwrap(),
            "output_path": output_path.to_str().unwrap()
        });

        let result = tool.execute(params).await.unwrap();
        assert!(result.is_error.is_none() || !result.is_error.unwrap());
        assert!(output_path.exists());
    }
}
