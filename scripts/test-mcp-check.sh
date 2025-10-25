#!/bin/bash
# Test the speckit_check tool via MCP server
# This simulates what Claude Code would send to the MCP server

echo "Testing spec-kit-mcp MCP server..."
echo ""

# Start the MCP server in the background
echo "Starting MCP server..."
./target/release/spec-kit-mcp &
MCP_PID=$!

# Give it a moment to start
sleep 2

# Check if it's still running
if ! kill -0 $MCP_PID 2>/dev/null; then
    echo "Error: MCP server failed to start"
    exit 1
fi

echo "MCP server started (PID: $MCP_PID)"
echo ""

# Send a JSON-RPC request to list tools
echo "Sending tools/list request..."
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | nc -w 5 localhost 3000 2>/dev/null || {
    echo "Note: Cannot test via network - MCP uses stdio transport"
    echo ""
    echo "The MCP server is designed to work over stdin/stdout (stdio transport)"
    echo "It should be configured in Claude Code's mcp.json file."
    echo ""
    echo "To test manually, try:"
    echo '  echo '"'"'{"jsonrpc":"2.0","id":1,"method":"tools/list"}'"'"' | ./target/release/spec-kit-mcp'
}

# Kill the server
echo ""
echo "Stopping MCP server..."
kill $MCP_PID 2>/dev/null
wait $MCP_PID 2>/dev/null

echo "Done!"
