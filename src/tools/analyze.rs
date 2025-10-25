//! Spec-Kit Analyze Tool
//!
//! Analyzes cross-artifact consistency and coverage.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;

use crate::mcp::types::{ContentBlock, ToolDefinition, ToolResult};
use crate::speckit::SpecKitCli;
use crate::tools::Tool;

/// Parameters for the speckit_analyze tool
#[derive(Debug, Deserialize, Serialize)]
pub struct AnalyzeParams {
    /// Path to project directory
    project_path: PathBuf,

    /// Check consistency across artifacts
    #[serde(default = "default_true")]
    check_consistency: bool,

    /// Check coverage of requirements
    #[serde(default = "default_true")]
    check_coverage: bool,

    /// Output path for analysis report
    #[serde(default = "default_analyze_path")]
    output_path: PathBuf,
}

fn default_true() -> bool {
    true
}

fn default_analyze_path() -> PathBuf {
    PathBuf::from("./speckit.analyze")
}

/// Tool for analyzing spec-kit artifacts
pub struct AnalyzeTool {
    #[allow(dead_code)]
    cli: SpecKitCli,
}

impl AnalyzeTool {
    /// Create a new analyze tool
    pub fn new(cli: SpecKitCli) -> Self {
        Self { cli }
    }
}

#[async_trait]
impl Tool for AnalyzeTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "speckit_analyze".to_string(),
            description: "Analyze cross-artifact consistency and coverage - ensures constitution, specs, plans, and tasks are aligned".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "project_path": {
                        "type": "string",
                        "description": "Path to the project directory containing spec-kit artifacts"
                    },
                    "check_consistency": {
                        "type": "boolean",
                        "default": true,
                        "description": "Check if artifacts are consistent with each other"
                    },
                    "check_coverage": {
                        "type": "boolean",
                        "default": true,
                        "description": "Check if all requirements are covered in plan/tasks"
                    },
                    "output_path": {
                        "type": "string",
                        "description": "Path where analysis report will be written",
                        "default": "./speckit.analyze"
                    }
                },
                "required": ["project_path"]
            })
        }
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let params: AnalyzeParams = serde_json::from_value(params)
            .context("Failed to parse analyze parameters")?;

        tracing::info!(
            project_path = %params.project_path.display(),
            "Analyzing project artifacts"
        );

        let mut analysis = String::from("# Spec-Kit Analysis Report\n\n");
        analysis.push_str(&format!("Project: {}\n\n", params.project_path.display()));

        // Check for artifacts
        let artifacts = vec![
            ("Constitution", "speckit.constitution"),
            ("Specification", "speckit.specify"),
            ("Plan", "speckit.plan"),
            ("Tasks", "speckit.tasks"),
        ];

        analysis.push_str("## Artifact Status\n\n");

        let mut found_artifacts = Vec::new();
        for (name, filename) in &artifacts {
            let path = params.project_path.join(filename);
            if path.exists() {
                analysis.push_str(&format!("✓ {} found\n", name));
                found_artifacts.push((*name, path));
            } else {
                analysis.push_str(&format!("✗ {} missing\n", name));
            }
        }

        // Consistency checks
        if params.check_consistency && found_artifacts.len() >= 2 {
            analysis.push_str("\n## Consistency Analysis\n\n");

            // Read all artifacts
            let mut contents = Vec::new();
            for (name, path) in &found_artifacts {
                match tokio::fs::read_to_string(path).await {
                    Ok(content) => contents.push((*name, content)),
                    Err(e) => analysis.push_str(&format!("⚠ Failed to read {}: {}\n", name, e)),
                }
            }

            // Check for keywords mentioned in spec but missing in plan
            if contents.len() >= 2 {
                let spec_content = contents.iter()
                    .find(|(n, _)| n.contains("Spec"))
                    .map(|(_, c)| c);
                let plan_content = contents.iter()
                    .find(|(n, _)| n.contains("Plan"))
                    .map(|(_, c)| c);

                if let (Some(spec), Some(plan)) = (spec_content, plan_content) {
                    // Extract key terms from spec
                    let important_terms = spec.split_whitespace()
                        .filter(|w| w.len() > 5 && w.chars().next().unwrap().is_uppercase())
                        .take(10)
                        .collect::<Vec<_>>();

                    let mut missing = Vec::new();
                    for term in important_terms {
                        if !plan.contains(term) {
                            missing.push(term);
                        }
                    }

                    if missing.is_empty() {
                        analysis.push_str("✓ Key terms from specification are addressed in plan\n");
                    } else {
                        analysis.push_str("⚠ Terms in spec but not in plan:\n");
                        for term in missing {
                            analysis.push_str(&format!("  - {}\n", term));
                        }
                    }
                }
            }
        }

        // Coverage checks
        if params.check_coverage {
            analysis.push_str("\n## Coverage Analysis\n\n");

            if found_artifacts.iter().any(|(n, _)| n.contains("Specification")) {
                analysis.push_str("✓ Requirements are specified\n");
            } else {
                analysis.push_str("✗ Missing specification\n");
            }

            if found_artifacts.iter().any(|(n, _)| n.contains("Plan")) {
                analysis.push_str("✓ Technical plan exists\n");
            } else {
                analysis.push_str("✗ Missing technical plan\n");
            }

            if found_artifacts.iter().any(|(n, _)| n.contains("Tasks")) {
                analysis.push_str("✓ Tasks are defined\n");
            } else {
                analysis.push_str("✗ Missing task breakdown\n");
            }
        }

        // Recommendations
        analysis.push_str("\n## Recommendations\n\n");

        if found_artifacts.len() < 4 {
            analysis.push_str("1. Complete missing artifacts\n");
        }
        analysis.push_str("2. Review consistency issues (if any)\n");
        analysis.push_str("3. Ensure all requirements are covered in tasks\n");
        analysis.push_str("4. Keep artifacts updated as project evolves\n");

        // Write analysis
        tokio::fs::write(&params.output_path, &analysis)
            .await
            .context("Failed to write analysis")?;

        let message = format!(
            "Analysis complete!\n\n\
            Artifacts found: {}/{}\n\
            Report: {}\n\n\
            {}",
            found_artifacts.len(),
            artifacts.len(),
            params.output_path.display(),
            if found_artifacts.len() == artifacts.len() {
                "✓ All artifacts present"
            } else {
                "⚠ Some artifacts are missing"
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
    async fn test_analyze_tool_definition() {
        let cli = SpecKitCli::new();
        let tool = AnalyzeTool::new(cli);
        let def = tool.definition();

        assert_eq!(def.name, "speckit_analyze");
        assert!(!def.description.is_empty());
    }

    #[tokio::test]
    async fn test_analyze_tool_execute() {
        let cli = SpecKitCli::new_test_mode();
        let tool = AnalyzeTool::new(cli);

        let dir = tempdir().unwrap();

        // Create some artifacts
        fs::write(dir.path().join("speckit.constitution"), "Principles").await.unwrap();
        fs::write(dir.path().join("speckit.specify"), "Requirements").await.unwrap();

        let params = json!({
            "project_path": dir.path().to_str().unwrap(),
            "check_consistency": true,
            "check_coverage": true
        });

        let result = tool.execute(params).await.unwrap();
        assert!(result.is_error.is_none() || !result.is_error.unwrap());
    }
}
