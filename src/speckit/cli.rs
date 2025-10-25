//! Spec-Kit CLI Executor
//!
//! Handles spawning and managing spec-kit CLI processes.

use anyhow::{Context, Result};
use async_process::{Command, Stdio};
use std::path::Path;
use std::time::Duration;
use tokio::time::timeout;

use super::errors::SpecKitError;

/// Result of executing a spec-kit command
#[derive(Debug, Clone)]
pub struct CommandResult {
    /// Standard output
    pub stdout: String,

    /// Standard error
    pub stderr: String,

    /// Exit code
    pub exit_code: i32,
}

impl CommandResult {
    /// Check if the command was successful
    pub fn is_success(&self) -> bool {
        self.exit_code == 0
    }

    /// Get a summary of the result
    pub fn summary(&self) -> String {
        if self.is_success() {
            format!("Success\n{}", self.stdout.trim())
        } else {
            format!(
                "Failed (exit code {})\n{}",
                self.exit_code,
                self.stderr.trim()
            )
        }
    }
}

/// Spec-kit CLI integration
#[derive(Debug, Clone)]
pub struct SpecKitCli {
    /// Path to the specify command
    cli_path: String,

    /// Path to Python interpreter (reserved for future use)
    #[allow(dead_code)]
    python_path: String,

    /// Default timeout for commands (in seconds)
    timeout_seconds: u64,

    /// Test mode flag
    test_mode: bool,
}

impl SpecKitCli {
    /// Create a new spec-kit CLI interface
    pub fn new() -> Self {
        Self {
            cli_path: "specify".to_string(),
            python_path: "python3".to_string(),
            timeout_seconds: 300, // 5 minutes
            test_mode: false,
        }
    }

    /// Create a test mode CLI (returns mock data)
    #[cfg(test)]
    pub fn new_test_mode() -> Self {
        Self {
            cli_path: "specify".to_string(),
            python_path: "python3".to_string(),
            timeout_seconds: 300,
            test_mode: true,
        }
    }

    /// Set the CLI path
    pub fn with_cli_path(mut self, path: impl Into<String>) -> Self {
        self.cli_path = path.into();
        self
    }

    /// Set the timeout
    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    }

    /// Check if spec-kit is installed
    pub async fn is_installed(&self) -> bool {
        if self.test_mode {
            return true;
        }

        Command::new(&self.cli_path)
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
            .map(|s| s.success())
            .unwrap_or(false)
    }

    /// Execute a spec-kit command
    async fn execute_command(&self, args: &[&str]) -> Result<CommandResult> {
        if self.test_mode {
            return Ok(CommandResult {
                stdout: "Test mode: command executed successfully".to_string(),
                stderr: String::new(),
                exit_code: 0,
            });
        }

        tracing::debug!(
            command = %self.cli_path,
            args = ?args,
            "Executing spec-kit command"
        );

        let command_future = Command::new(&self.cli_path)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        let output = timeout(Duration::from_secs(self.timeout_seconds), command_future)
            .await
            .context("Command timeout")?
            .context("Failed to execute command")?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);

        let result = CommandResult {
            stdout,
            stderr,
            exit_code,
        };

        if !result.is_success() {
            tracing::warn!(
                exit_code = result.exit_code,
                stderr = %result.stderr,
                "Spec-kit command failed"
            );
        }

        Ok(result)
    }

    /// Initialize a new spec-kit project
    pub async fn init(&self, project_name: &str, path: &Path) -> Result<CommandResult> {
        let path_str = path
            .to_str()
            .ok_or_else(|| SpecKitError::InvalidPath("Path contains invalid UTF-8".to_string()))?;

        let result = self
            .execute_command(&["init", project_name, "--path", path_str])
            .await?;

        if !result.is_success() {
            return Err(SpecKitError::command_failed(
                format!("specify init {}", project_name),
                &result.stderr,
                result.exit_code,
            )
            .into());
        }

        Ok(result)
    }

    /// Create a constitution file
    pub async fn constitution(&self, content: &str, output_path: &Path) -> Result<CommandResult> {
        // For now, we'll write directly to the file since spec-kit
        // accepts input via stdin or prompts
        tokio::fs::write(output_path, content)
            .await
            .context("Failed to write constitution file")?;

        Ok(CommandResult {
            stdout: format!("Constitution written to {}", output_path.display()),
            stderr: String::new(),
            exit_code: 0,
        })
    }

    /// Create a specification file
    pub async fn specify(
        &self,
        requirements: &str,
        output_path: &Path,
        _format: &str,
    ) -> Result<CommandResult> {
        // Write requirements to file
        // Spec-kit typically uses interactive prompts or file input
        tokio::fs::write(output_path, requirements)
            .await
            .context("Failed to write specification file")?;

        Ok(CommandResult {
            stdout: format!("Specification written to {}", output_path.display()),
            stderr: String::new(),
            exit_code: 0,
        })
    }

    /// Create a technical plan
    pub async fn plan(&self, spec_file: &Path, output_path: &Path) -> Result<CommandResult> {
        let spec_str = spec_file.to_str().ok_or_else(|| {
            SpecKitError::InvalidPath("Spec file path contains invalid UTF-8".to_string())
        })?;

        let output_str = output_path.to_str().ok_or_else(|| {
            SpecKitError::InvalidPath("Output path contains invalid UTF-8".to_string())
        })?;

        let result = self
            .execute_command(&["plan", "--spec", spec_str, "--output", output_str])
            .await?;

        if !result.is_success() {
            return Err(SpecKitError::command_failed(
                "specify plan",
                &result.stderr,
                result.exit_code,
            )
            .into());
        }

        Ok(result)
    }

    /// Generate task list
    pub async fn tasks(&self, plan_file: &Path, output_path: &Path) -> Result<CommandResult> {
        let plan_str = plan_file.to_str().ok_or_else(|| {
            SpecKitError::InvalidPath("Plan file path contains invalid UTF-8".to_string())
        })?;

        let output_str = output_path.to_str().ok_or_else(|| {
            SpecKitError::InvalidPath("Output path contains invalid UTF-8".to_string())
        })?;

        let result = self
            .execute_command(&["tasks", "--plan", plan_str, "--output", output_str])
            .await?;

        if !result.is_success() {
            return Err(SpecKitError::command_failed(
                "specify tasks",
                &result.stderr,
                result.exit_code,
            )
            .into());
        }

        Ok(result)
    }

    /// Analyze project consistency
    pub async fn analyze(&self, project_path: &Path) -> Result<CommandResult> {
        let path_str = project_path.to_str().ok_or_else(|| {
            SpecKitError::InvalidPath("Project path contains invalid UTF-8".to_string())
        })?;

        let result = self
            .execute_command(&["analyze", "--path", path_str])
            .await?;

        if !result.is_success() {
            return Err(SpecKitError::command_failed(
                "specify analyze",
                &result.stderr,
                result.exit_code,
            )
            .into());
        }

        Ok(result)
    }
}

impl Default for SpecKitCli {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_cli_creation() {
        let cli = SpecKitCli::new();
        assert_eq!(cli.timeout_seconds, 300);
    }

    #[tokio::test]
    async fn test_constitution_write() {
        let cli = SpecKitCli::new_test_mode();
        let dir = tempdir().unwrap();
        let output_path = dir.path().join("constitution.md");

        let result = cli
            .constitution("Test constitution", &output_path)
            .await
            .unwrap();

        assert!(result.is_success());
        assert!(output_path.exists());
    }

    #[tokio::test]
    async fn test_specify_write() {
        let cli = SpecKitCli::new_test_mode();
        let dir = tempdir().unwrap();
        let output_path = dir.path().join("specification.md");

        let result = cli
            .specify("Test requirements", &output_path, "markdown")
            .await
            .unwrap();

        assert!(result.is_success());
        assert!(output_path.exists());
    }
}
