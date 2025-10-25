//! Spec-Kit Checklist Tool
//!
//! Generates validation checklists from specifications.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;

use crate::mcp::types::{ContentBlock, ToolDefinition, ToolResult};
use crate::speckit::SpecKitCli;
use crate::tools::Tool;

/// Parameters for the speckit_checklist tool
#[derive(Debug, Deserialize, Serialize)]
pub struct ChecklistParams {
    /// Path to specification file
    spec_file: PathBuf,

    /// Include implementation checklist items
    #[serde(default = "default_true")]
    include_implementation: bool,

    /// Include testing checklist items
    #[serde(default = "default_true")]
    include_testing: bool,

    /// Output path for checklist
    #[serde(default = "default_checklist_path")]
    output_path: PathBuf,
}

fn default_true() -> bool {
    true
}

fn default_checklist_path() -> PathBuf {
    PathBuf::from("./speckit.checklist")
}

/// Tool for generating validation checklists
pub struct ChecklistTool {
    #[allow(dead_code)]
    cli: SpecKitCli,
}

impl ChecklistTool {
    /// Create a new checklist tool
    pub fn new(cli: SpecKitCli) -> Self {
        Self { cli }
    }
}

#[async_trait]
impl Tool for ChecklistTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "speckit_checklist".to_string(),
            description: "Generate a validation checklist from the specification to ensure all requirements are met".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "spec_file": {
                        "type": "string",
                        "description": "Path to the specification file"
                    },
                    "include_implementation": {
                        "type": "boolean",
                        "default": true,
                        "description": "Include implementation checklist items"
                    },
                    "include_testing": {
                        "type": "boolean",
                        "default": true,
                        "description": "Include testing checklist items"
                    },
                    "output_path": {
                        "type": "string",
                        "description": "Path where checklist will be written",
                        "default": "./speckit.checklist"
                    }
                },
                "required": ["spec_file"]
            })
        }
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let params: ChecklistParams =
            serde_json::from_value(params).context("Failed to parse checklist parameters")?;

        tracing::info!(
            spec_file = %params.spec_file.display(),
            "Generating validation checklist"
        );

        // Read specification
        let spec_content = tokio::fs::read_to_string(&params.spec_file)
            .await
            .context("Failed to read specification file")?;

        // Generate checklist
        let mut checklist = String::from("# Implementation & Validation Checklist\n\n");
        checklist.push_str(&format!("Based on: {}\n\n", params.spec_file.display()));

        // Requirements checklist
        checklist.push_str("## Requirements Validation\n\n");

        // Extract requirements (simple heuristic)
        let mut req_count = 0;
        for line in spec_content.lines() {
            if line.trim().starts_with('-')
                || line.trim().starts_with('*')
                || line.contains("shall")
                || line.contains("must")
                || line.contains("should")
            {
                req_count += 1;
                let requirement = line
                    .trim()
                    .trim_start_matches('-')
                    .trim_start_matches('*')
                    .trim();
                if !requirement.is_empty() && requirement.len() < 100 {
                    checklist.push_str(&format!("- [ ] {}\n", requirement));
                }
            }
        }

        if req_count == 0 {
            checklist.push_str("- [ ] All specified requirements are implemented\n");
            checklist.push_str("- [ ] Edge cases are handled\n");
            checklist.push_str("- [ ] Error conditions are addressed\n");
        }

        // Implementation checklist
        if params.include_implementation {
            checklist.push_str("\n## Implementation Checklist\n\n");
            checklist.push_str("- [ ] Code follows project style guide\n");
            checklist.push_str("- [ ] Functions have clear documentation\n");
            checklist.push_str("- [ ] Error handling is comprehensive\n");
            checklist.push_str("- [ ] Input validation is performed\n");
            checklist.push_str("- [ ] Logging is appropriate\n");
            checklist.push_str("- [ ] Performance is acceptable\n");
            checklist.push_str("- [ ] Security considerations addressed\n");
        }

        // Testing checklist
        if params.include_testing {
            checklist.push_str("\n## Testing Checklist\n\n");
            checklist.push_str("- [ ] Unit tests written for all functions\n");
            checklist.push_str("- [ ] Integration tests cover main workflows\n");
            checklist.push_str("- [ ] Edge cases are tested\n");
            checklist.push_str("- [ ] Error conditions are tested\n");
            checklist.push_str("- [ ] Performance tests (if applicable)\n");
            checklist.push_str("- [ ] All tests pass\n");
            checklist.push_str("- [ ] Test coverage >80%\n");
        }

        // Quality checklist
        checklist.push_str("\n## Quality Assurance\n\n");
        checklist.push_str("- [ ] Code review completed\n");
        checklist.push_str("- [ ] Documentation updated\n");
        checklist.push_str("- [ ] CHANGELOG.md updated\n");
        checklist.push_str("- [ ] No compiler warnings\n");
        checklist.push_str("- [ ] Linter passes (clippy, etc.)\n");
        checklist.push_str("- [ ] Dependencies are up to date\n");

        // Deployment checklist
        checklist.push_str("\n## Deployment Readiness\n\n");
        checklist.push_str("- [ ] All tests pass in CI\n");
        checklist.push_str("- [ ] Version number updated\n");
        checklist.push_str("- [ ] Release notes prepared\n");
        checklist.push_str("- [ ] Breaking changes documented\n");
        checklist.push_str("- [ ] Migration guide provided (if needed)\n");

        // Write checklist
        tokio::fs::write(&params.output_path, &checklist)
            .await
            .context("Failed to write checklist")?;

        let total_items = checklist.matches("- [ ]").count();

        let message = format!(
            "Validation checklist generated!\n\n\
            Source: {}\n\
            Total items: {}\n\
            Output: {}\n\n\
            Use this checklist to ensure all requirements are met and\n\
            quality standards are maintained throughout implementation.",
            params.spec_file.display(),
            total_items,
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
    async fn test_checklist_tool_definition() {
        let cli = SpecKitCli::new();
        let tool = ChecklistTool::new(cli);
        let def = tool.definition();

        assert_eq!(def.name, "speckit_checklist");
        assert!(!def.description.is_empty());
    }

    #[tokio::test]
    async fn test_checklist_tool_execute() {
        let cli = SpecKitCli::new_test_mode();
        let tool = ChecklistTool::new(cli);

        let dir = tempdir().unwrap();
        let spec_file = dir.path().join("spec.md");
        let output_path = dir.path().join("checklist.md");

        // Create spec with requirements
        fs::write(
            &spec_file,
            "- User must login\n- System shall validate input\n- Should handle errors",
        )
        .await
        .unwrap();

        let params = json!({
            "spec_file": spec_file.to_str().unwrap(),
            "include_implementation": true,
            "include_testing": true,
            "output_path": output_path.to_str().unwrap()
        });

        let result = tool.execute(params).await.unwrap();
        assert!(result.is_error.is_none() || !result.is_error.unwrap());
        assert!(output_path.exists());

        // Verify checklist has items
        let content = fs::read_to_string(output_path).await.unwrap();
        assert!(content.contains("- [ ]"));
    }
}
