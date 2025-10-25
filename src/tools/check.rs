//! Spec-Kit Check Tool
//!
//! Validates that required tools are installed.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::mcp::types::{ContentBlock, ToolDefinition, ToolResult};
use crate::speckit::SpecKitCli;
use crate::tools::Tool;

/// Parameters for the speckit_check tool
#[derive(Debug, Deserialize, Serialize)]
pub struct CheckParams {
    /// Check for spec-kit CLI
    #[serde(default = "default_true")]
    check_speckit: bool,

    /// Check for git
    #[serde(default = "default_true")]
    check_git: bool,

    /// Check for common AI coding assistants
    #[serde(default = "default_true")]
    check_ai_tools: bool,
}

fn default_true() -> bool {
    true
}

impl Default for CheckParams {
    fn default() -> Self {
        Self {
            check_speckit: true,
            check_git: true,
            check_ai_tools: true,
        }
    }
}

/// Tool for checking required tool installations
pub struct CheckTool {
    #[allow(dead_code)]
    cli: SpecKitCli,
}

impl CheckTool {
    /// Create a new check tool
    pub fn new(cli: SpecKitCli) -> Self {
        Self { cli }
    }

    /// Check if a command is available
    async fn check_command(&self, command: &str) -> bool {
        async_process::Command::new("which")
            .arg(command)
            .stdout(async_process::Stdio::null())
            .stderr(async_process::Stdio::null())
            .status()
            .await
            .map(|s| s.success())
            .unwrap_or(false)
    }
}

#[async_trait]
impl Tool for CheckTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "speckit_check".to_string(),
            description: "Validate that required tools are installed for spec-kit development"
                .to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "check_speckit": {
                        "type": "boolean",
                        "default": true,
                        "description": "Check if spec-kit CLI is installed"
                    },
                    "check_git": {
                        "type": "boolean",
                        "default": true,
                        "description": "Check if git is installed"
                    },
                    "check_ai_tools": {
                        "type": "boolean",
                        "default": true,
                        "description": "Check for AI coding assistants (claude, cursor, etc.)"
                    }
                },
                "required": []
            }),
        }
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let params: CheckParams =
            if params.is_null() || params.as_object().is_some_and(|o| o.is_empty()) {
                CheckParams::default()
            } else {
                serde_json::from_value(params).context("Failed to parse check parameters")?
            };

        tracing::info!("Checking tool installations");

        let mut report = String::from("# Tool Installation Check\n\n");
        let mut all_good = true;

        // Check spec-kit CLI
        if params.check_speckit {
            report.push_str("## Spec-Kit CLI\n\n");
            let has_specify = self.check_command("specify").await;
            if has_specify {
                report.push_str("✅ `specify` command is available\n");
            } else {
                report.push_str("❌ `specify` command not found\n");
                report.push_str("   Install with: `uv tool install specify-cli`\n");
                all_good = false;
            }
            report.push('\n');
        }

        // Check git
        if params.check_git {
            report.push_str("## Version Control\n\n");
            let has_git = self.check_command("git").await;
            if has_git {
                report.push_str("✅ `git` is available\n");
            } else {
                report.push_str("❌ `git` not found\n");
                report.push_str("   Install from: https://git-scm.com/\n");
                all_good = false;
            }
            report.push('\n');
        }

        // Check AI tools
        if params.check_ai_tools {
            report.push_str("## AI Coding Assistants\n\n");

            let ai_tools = vec![
                ("code", "Visual Studio Code"),
                ("code-insiders", "VS Code Insiders"),
                ("cursor", "Cursor"),
                ("windsurf", "Windsurf"),
            ];

            let mut found_any = false;
            for (cmd, name) in &ai_tools {
                if self.check_command(cmd).await {
                    report.push_str(&format!("✅ `{}` ({}) is available\n", cmd, name));
                    found_any = true;
                }
            }

            if !found_any {
                report.push_str("⚠️  No AI coding assistants found\n");
                report.push_str("   Consider installing:\n");
                report.push_str("   - VS Code: https://code.visualstudio.com/\n");
                report.push_str("   - Cursor: https://cursor.sh/\n");
                report.push_str("   - Windsurf: https://codeium.com/windsurf\n");
            }
            report.push('\n');
        }

        // Summary
        report.push_str("## Summary\n\n");
        if all_good {
            report.push_str("✅ All required tools are installed!\n");
            report.push_str("\nYou're ready to use spec-kit for spec-driven development.\n");
        } else {
            report.push_str("⚠️  Some required tools are missing.\n");
            report.push_str("\nPlease install the missing tools to use spec-kit effectively.\n");
        }

        Ok(ToolResult {
            content: vec![ContentBlock::text(report)],
            is_error: Some(!all_good),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_tool_definition() {
        let cli = SpecKitCli::new();
        let tool = CheckTool::new(cli);
        let def = tool.definition();

        assert_eq!(def.name, "speckit_check");
        assert!(!def.description.is_empty());
    }

    #[tokio::test]
    async fn test_check_tool_execute() {
        let cli = SpecKitCli::new_test_mode();
        let tool = CheckTool::new(cli);

        // Test with default params
        let result = tool.execute(json!({})).await.unwrap();
        assert!(!result.content.is_empty());

        // Test with specific params
        let params = json!({
            "check_speckit": true,
            "check_git": true,
            "check_ai_tools": false
        });

        let result = tool.execute(params).await.unwrap();
        assert!(!result.content.is_empty());
    }

    #[tokio::test]
    async fn test_check_command() {
        let cli = SpecKitCli::new_test_mode();
        let tool = CheckTool::new(cli);

        // Should find common commands
        assert!(tool.check_command("ls").await || tool.check_command("dir").await);
    }
}
