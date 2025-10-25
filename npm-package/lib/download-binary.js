#!/usr/bin/env node

/**
 * Download Binary Script
 *
 * Downloads the appropriate pre-built binary for the current platform.
 * This runs as a postinstall script.
 */

const https = require('https');
const fs = require('fs');
const path = require('path');
const { createHash } = require('crypto');

const GITHUB_REPO = 'yourusername/spec-kit-mcp';
const VERSION = require('../package.json').version;

// Platform-specific binary information
function getPlatformInfo() {
  const platform = process.platform;
  const arch = process.arch;

  let target;
  let binaryName = 'spec-kit-mcp';

  if (platform === 'darwin') {
    if (arch === 'arm64') {
      target = 'aarch64-apple-darwin';
    } else {
      target = 'x86_64-apple-darwin';
    }
  } else if (platform === 'linux') {
    if (arch === 'x64') {
      target = 'x86_64-unknown-linux-gnu';
    } else {
      console.warn(`Unsupported Linux architecture: ${arch}`);
      return null;
    }
  } else if (platform === 'win32') {
    binaryName = 'spec-kit-mcp.exe';
    if (arch === 'x64') {
      target = 'x86_64-pc-windows-msvc';
    } else {
      console.warn(`Unsupported Windows architecture: ${arch}`);
      return null;
    }
  } else {
    console.warn(`Unsupported platform: ${platform}`);
    return null;
  }

  return { target, binaryName };
}

// Download file from URL
function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);

    https.get(url, {
      headers: {
        'User-Agent': 'spec-kit-mcp-installer'
      }
    }, (response) => {
      // Handle redirects
      if (response.statusCode === 302 || response.statusCode === 301) {
        file.close();
        fs.unlinkSync(dest);
        return downloadFile(response.headers.location, dest)
          .then(resolve)
          .catch(reject);
      }

      if (response.statusCode !== 200) {
        file.close();
        fs.unlinkSync(dest);
        return reject(new Error(`Failed to download: ${response.statusCode}`));
      }

      response.pipe(file);

      file.on('finish', () => {
        file.close();
        resolve();
      });
    }).on('error', (err) => {
      file.close();
      fs.unlinkSync(dest);
      reject(err);
    });
  });
}

// Main installation logic
async function install() {
  console.log('Installing spec-kit-mcp binary...');

  const platformInfo = getPlatformInfo();

  if (!platformInfo) {
    console.warn('No pre-built binary available for your platform.');
    console.warn('You will need to build from source:');
    console.warn('  git clone https://github.com/yourusername/spec-kit-mcp.git');
    console.warn('  cd spec-kit-mcp');
    console.warn('  cargo build --release');
    return;
  }

  const { target, binaryName } = platformInfo;

  // Create bin directory
  const binDir = path.join(__dirname, '..', 'bin');
  if (!fs.existsSync(binDir)) {
    fs.mkdirSync(binDir, { recursive: true });
  }

  const binaryPath = path.join(binDir, binaryName);

  // Check if binary already exists
  if (fs.existsSync(binaryPath)) {
    console.log('Binary already exists, skipping download.');
    return;
  }

  // Construct download URL
  const url = `https://github.com/${GITHUB_REPO}/releases/download/v${VERSION}/spec-kit-mcp-${target}${binaryName.endsWith('.exe') ? '.exe' : ''}`;

  console.log(`Downloading from: ${url}`);

  try {
    await downloadFile(url, binaryPath);

    // Make executable on Unix-like systems
    if (process.platform !== 'win32') {
      fs.chmodSync(binaryPath, 0o755);
    }

    console.log('✓ Binary downloaded and installed successfully!');
    console.log(`  Location: ${binaryPath}`);
  } catch (error) {
    console.error('Failed to download binary:', error.message);
    console.error('\nYou can build from source instead:');
    console.error('  git clone https://github.com/yourusername/spec-kit-mcp.git');
    console.error('  cd spec-kit-mcp');
    console.error('  cargo build --release');
    console.error('  cargo install --path .');

    // Don't fail the installation, just warn
    // Users can still build from source
  }
}

// Run installation if this is the main module
if (require.main === module) {
  install().catch(err => {
    console.error('Installation error:', err);
    // Don't exit with error code to allow installation to continue
  });
}

module.exports = { install };
