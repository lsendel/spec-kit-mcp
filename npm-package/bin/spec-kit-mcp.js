#!/usr/bin/env node

/**
 * Spec-Kit MCP Server Entry Point
 *
 * On macOS/Linux: launches the native Rust binary (fast, <100ms cold start).
 * On Windows or when no binary is found: falls back to the pure Node.js
 * implementation — same 10 tools, no compilation required.
 */

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

// Determine the binary path based on platform and architecture.
// Returns `null` when the native binary is not available.
function getBinaryPath() {
  const platform = process.platform;
  const arch = process.arch;

  let binaryName = 'spec-kit-mcp';
  let platformDir;

  if (platform === 'win32') {
    binaryName = 'spec-kit-mcp.exe';
    platformDir = `mcp-win32-${arch}`;
  } else if (platform === 'darwin') {
    if (arch === 'arm64') {
      platformDir = 'mcp-darwin-arm64';
    } else {
      platformDir = 'mcp-darwin-x64';
    }
  } else if (platform === 'linux') {
    platformDir = `mcp-linux-${arch}`;
  } else {
    return null; // fallback to Node.js
  }

  const locations = [
    path.join(__dirname, binaryName),
    path.join(__dirname, '..', '..', platformDir, 'bin', binaryName),
    path.join(__dirname, '..', '..', 'target', 'release', binaryName),
  ];

  for (const location of locations) {
    if (fs.existsSync(location)) return location;
  }

  // Check PATH for a cargo-installed binary (Unix only — no native
  // binary has been published for Windows).
  if (process.platform !== 'win32') {
    try {
      const { execSync } = require('child_process');
      execSync('command -v spec-kit-mcp', { stdio: 'ignore' });
      return 'spec-kit-mcp';
    } catch (e) { /* not in PATH */ }
  }

  return null;
}

const binaryPath = getBinaryPath();

if (binaryPath) {
  // ── Native binary mode (macOS / Linux) ──
  const args = process.argv.slice(2);
  const child = spawn(binaryPath, args, { stdio: 'inherit', shell: false });

  child.on('exit', (code) => process.exit(code || 0));
  child.on('error', (err) => {
    process.stderr.write(`Failed to start spec-kit-mcp binary: ${err.message}\n`);
    process.exit(1);
  });

  process.on('SIGINT', () => child.kill('SIGINT'));
  process.on('SIGTERM', () => child.kill('SIGTERM'));
} else {
  // ── Node.js fallback (Windows or no binary) ──
  process.stderr.write('spec-kit-mcp [node] starting…\n');
  require(path.join(__dirname, '..', 'lib', 'mcp-server', 'index.js'));
}
