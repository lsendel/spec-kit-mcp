#!/usr/bin/env node

/**
 * Spec-Kit MCP Server Entry Point
 *
 * This script launches the spec-kit-mcp binary for the current platform.
 */

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

// Determine the binary path based on platform and architecture
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
    console.error(`Unsupported platform: ${platform}`);
    process.exit(1);
  }

  // Check multiple possible locations
  const locations = [
    // Installed as dependency (in node_modules/@speckit/)
    path.join(__dirname, '..', '..', platformDir, 'bin', binaryName),
    // Installed globally
    path.join(__dirname, '..', 'bin', binaryName),
    // Development (built locally)
    path.join(__dirname, '..', '..', 'target', 'release', binaryName),
  ];

  for (const location of locations) {
    if (fs.existsSync(location)) {
      return location;
    }
  }

  console.error('Error: spec-kit-mcp binary not found!');
  console.error('Searched locations:');
  locations.forEach(loc => console.error(`  - ${loc}`));
  console.error('\nPlease ensure the package was installed correctly.');
  console.error('You may need to run: npm install --force');
  process.exit(1);
}

// Get the binary path
const binaryPath = getBinaryPath();

// Forward all arguments to the binary
const args = process.argv.slice(2);

// Spawn the binary
const child = spawn(binaryPath, args, {
  stdio: 'inherit',
  shell: false,
});

// Forward exit code
child.on('exit', (code) => {
  process.exit(code || 0);
});

// Handle errors
child.on('error', (err) => {
  console.error('Failed to start spec-kit-mcp:', err.message);
  process.exit(1);
});

// Handle signals
process.on('SIGINT', () => {
  child.kill('SIGINT');
});

process.on('SIGTERM', () => {
  child.kill('SIGTERM');
});
