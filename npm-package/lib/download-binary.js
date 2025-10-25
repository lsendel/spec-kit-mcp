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

const GITHUB_REPO = 'lsendel/spec-kit-mcp';
const VERSION = require('../package.json').version;

// Platform-specific binary information
function getPlatformInfo() {
  const platform = process.platform;
  const arch = process.arch;

  let archiveName;
  let binaryName = 'spec-kit-mcp';

  if (platform === 'darwin') {
    if (arch === 'arm64') {
      archiveName = 'darwin-arm64';
    } else if (arch === 'x64') {
      archiveName = 'darwin-x64';
    } else {
      console.warn(`Unsupported macOS architecture: ${arch}`);
      return null;
    }
  } else if (platform === 'linux') {
    if (arch === 'x64') {
      archiveName = 'linux-x64';
    } else {
      console.warn(`Unsupported Linux architecture: ${arch}`);
      return null;
    }
  } else if (platform === 'win32') {
    binaryName = 'spec-kit-mcp.exe';
    if (arch === 'x64') {
      archiveName = 'win32-x64';
    } else {
      console.warn(`Unsupported Windows architecture: ${arch}`);
      return null;
    }
  } else {
    console.warn(`Unsupported platform: ${platform}`);
    return null;
  }

  return { archiveName, binaryName };
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

// Extract tar.gz archive
async function extractTarGz(archivePath, destDir) {
  const { exec } = require('child_process');
  const { promisify } = require('util');
  const execAsync = promisify(exec);

  try {
    await execAsync(`tar -xzf "${archivePath}" -C "${destDir}"`);
  } catch (error) {
    throw new Error(`Failed to extract archive: ${error.message}`);
  }
}

// Main installation logic
async function install() {
  console.log('Installing spec-kit-mcp binary...');

  const platformInfo = getPlatformInfo();

  if (!platformInfo) {
    console.warn('No pre-built binary available for your platform.');
    console.warn('You will need to build from source:');
    console.warn('  git clone https://github.com/lsendel/spec-kit-mcp.git');
    console.warn('  cd spec-kit-mcp');
    console.warn('  cargo build --release');
    return;
  }

  const { archiveName, binaryName } = platformInfo;

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

  // Construct download URL for tar.gz archive
  const archiveFilename = `spec-kit-mcp-${archiveName}.tar.gz`;
  const url = `https://github.com/${GITHUB_REPO}/releases/download/v${VERSION}/${archiveFilename}`;
  const archivePath = path.join(binDir, archiveFilename);

  console.log(`Downloading from: ${url}`);

  try {
    // Download the archive
    await downloadFile(url, archivePath);
    console.log('✓ Archive downloaded');

    // Extract the archive
    console.log('Extracting archive...');
    await extractTarGz(archivePath, binDir);

    // Remove the archive
    fs.unlinkSync(archivePath);

    // Make executable on Unix-like systems
    if (process.platform !== 'win32' && fs.existsSync(binaryPath)) {
      fs.chmodSync(binaryPath, 0o755);
    }

    // Verify binary exists
    if (!fs.existsSync(binaryPath)) {
      throw new Error('Binary not found after extraction');
    }

    console.log('✓ Binary installed successfully!');
    console.log(`  Location: ${binaryPath}`);
  } catch (error) {
    console.error('Failed to install binary:', error.message);
    console.error('\nYou can install using cargo instead:');
    console.error('  cargo install spec-kit-mcp');
    console.error('\nOr build from source:');
    console.error('  git clone https://github.com/lsendel/spec-kit-mcp.git');
    console.error('  cd spec-kit-mcp');
    console.error('  cargo build --release');

    // Don't fail the installation, just warn
    // Users can still use cargo install
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
