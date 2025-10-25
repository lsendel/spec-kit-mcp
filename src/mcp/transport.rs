//! Stdio Transport Layer for MCP
//!
//! Handles reading and writing JSON-RPC messages over stdin/stdout.

use anyhow::{Context, Result};
use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::io::{self, Stdin, Stdout};

use super::types::{JsonRpcRequest, JsonRpcResponse};

/// Stdio transport for MCP communication
pub struct StdioTransport {
    reader: BufReader<Stdin>,
    writer: Stdout,
}

impl StdioTransport {
    /// Create a new stdio transport
    pub fn new() -> Self {
        Self {
            reader: BufReader::new(io::stdin()),
            writer: io::stdout(),
        }
    }

    /// Read a JSON-RPC request from stdin
    pub async fn read_request(&mut self) -> Result<JsonRpcRequest> {
        let mut line = String::new();
        self.reader
            .read_line(&mut line)
            .await
            .context("Failed to read from stdin")?;

        if line.is_empty() {
            anyhow::bail!("EOF reached");
        }

        let request: JsonRpcRequest =
            serde_json::from_str(&line).context("Failed to parse JSON-RPC request")?;

        tracing::debug!(
            method = %request.method,
            id = ?request.id,
            "Received request"
        );

        Ok(request)
    }

    /// Write a JSON-RPC response to stdout
    pub async fn write_response(&mut self, response: JsonRpcResponse) -> Result<()> {
        let json = serde_json::to_string(&response).context("Failed to serialize response")?;

        tracing::debug!(
            id = ?response.id,
            has_error = response.error.is_some(),
            "Sending response"
        );

        self.writer
            .write_all(json.as_bytes())
            .await
            .context("Failed to write to stdout")?;

        self.writer
            .write_all(b"\n")
            .await
            .context("Failed to write newline")?;

        self.writer
            .flush()
            .await
            .context("Failed to flush stdout")?;

        Ok(())
    }

    /// Write a raw JSON value to stdout (for non-standard messages)
    pub async fn write_value(&mut self, value: Value) -> Result<()> {
        let json = serde_json::to_string(&value).context("Failed to serialize value")?;

        self.writer
            .write_all(json.as_bytes())
            .await
            .context("Failed to write to stdout")?;

        self.writer
            .write_all(b"\n")
            .await
            .context("Failed to write newline")?;

        self.writer
            .flush()
            .await
            .context("Failed to flush stdout")?;

        Ok(())
    }
}

impl Default for StdioTransport {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_creation() {
        let transport = StdioTransport::new();
        assert!(std::mem::size_of_val(&transport) > 0);
    }
}
