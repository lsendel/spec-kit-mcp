//! MCP Protocol Handler
//!
//! Handles JSON-RPC protocol validation and routing.

use anyhow::{Context, Result};
use serde_json::{json, Value};

use super::types::*;

/// Protocol handler for MCP messages
pub struct ProtocolHandler;

impl ProtocolHandler {
    /// Create a new protocol handler
    pub fn new() -> Self {
        Self
    }

    /// Validate a JSON-RPC request
    pub fn validate_request(&self, request: &JsonRpcRequest) -> Result<()> {
        // Check JSON-RPC version
        if request.jsonrpc != "2.0" {
            anyhow::bail!("Invalid JSON-RPC version: {}", request.jsonrpc);
        }

        // Method must not be empty
        if request.method.is_empty() {
            anyhow::bail!("Method cannot be empty");
        }

        Ok(())
    }

    /// Parse tool call parameters
    pub fn parse_tool_call(&self, params: Option<Value>) -> Result<ToolCallParams> {
        let params = params.ok_or_else(|| anyhow::anyhow!("Missing parameters"))?;

        serde_json::from_value(params).context("Failed to parse tool call parameters")
    }

    /// Create a tool list response
    pub fn create_tool_list_response(
        &self,
        id: RequestId,
        tools: Vec<ToolDefinition>,
    ) -> JsonRpcResponse {
        JsonRpcResponse::success(
            id,
            json!({
                "tools": tools
            }),
        )
    }

    /// Create a tool result response
    pub fn create_tool_result_response(
        &self,
        id: RequestId,
        result: ToolResult,
    ) -> JsonRpcResponse {
        JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
    }

    /// Create an error response from an error
    pub fn create_error_response(&self, id: RequestId, error: anyhow::Error) -> JsonRpcResponse {
        let error_msg = format!("{:#}", error);

        tracing::error!(error = %error_msg, "Request failed");

        JsonRpcResponse::error(id, JsonRpcError::internal_error(error_msg))
    }

    /// Handle initialization request
    pub fn handle_initialize(&self, id: RequestId) -> JsonRpcResponse {
        tracing::info!("Handling initialize request");

        JsonRpcResponse::success(
            id,
            json!({
                "protocolVersion": "2024-11-05",
                "serverInfo": {
                    "name": "spec-kit-mcp",
                    "version": env!("CARGO_PKG_VERSION")
                },
                "capabilities": {
                    "tools": {}
                }
            }),
        )
    }

    /// Handle ping request
    pub fn handle_ping(&self, id: RequestId) -> JsonRpcResponse {
        JsonRpcResponse::success(id, json!({}))
    }
}

impl Default for ProtocolHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_request_valid() {
        let handler = ProtocolHandler::new();
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: RequestId::Number(1),
            method: "test".to_string(),
            params: None,
        };

        assert!(handler.validate_request(&request).is_ok());
    }

    #[test]
    fn test_validate_request_invalid_version() {
        let handler = ProtocolHandler::new();
        let request = JsonRpcRequest {
            jsonrpc: "1.0".to_string(),
            id: RequestId::Number(1),
            method: "test".to_string(),
            params: None,
        };

        assert!(handler.validate_request(&request).is_err());
    }

    #[test]
    fn test_validate_request_empty_method() {
        let handler = ProtocolHandler::new();
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: RequestId::Number(1),
            method: "".to_string(),
            params: None,
        };

        assert!(handler.validate_request(&request).is_err());
    }

    #[test]
    fn test_initialize_response() {
        let handler = ProtocolHandler::new();
        let response = handler.handle_initialize(RequestId::Number(1));

        assert!(response.error.is_none());
        assert!(response.result.is_some());

        let result = response.result.unwrap();
        assert_eq!(result["serverInfo"]["name"], "spec-kit-mcp");
    }

    #[test]
    fn test_ping_response() {
        let handler = ProtocolHandler::new();
        let response = handler.handle_ping(RequestId::Number(1));

        assert!(response.error.is_none());
        assert!(response.result.is_some());
    }
}
