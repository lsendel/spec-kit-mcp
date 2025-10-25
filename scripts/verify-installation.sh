#!/bin/bash
# Spec-Kit MCP Installation Verification Script
# This script verifies that all prerequisites are installed and working

# Don't exit on errors - we want to check all tools
set +e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}================================================${NC}"
echo -e "${BLUE}  Spec-Kit MCP Installation Verification${NC}"
echo -e "${BLUE}================================================${NC}"
echo ""

ERRORS=0
WARNINGS=0

# Function to check command availability
check_command() {
    local cmd=$1
    local name=$2
    local install_url=$3
    local required=$4

    echo -n "Checking for $name... "
    if command -v "$cmd" &> /dev/null; then
        local version=$("$cmd" --version 2>&1 | head -n 1)
        echo -e "${GREEN}✓${NC} Found: $version"
        return 0
    else
        if [ "$required" = "true" ]; then
            echo -e "${RED}✗${NC} Not found (REQUIRED)"
            echo -e "   Install from: $install_url"
            ((ERRORS++))
        else
            echo -e "${YELLOW}⚠${NC} Not found (optional)"
            echo -e "   Install from: $install_url"
            ((WARNINGS++))
        fi
        return 1
    fi
}

# Function to check Python version
check_python_version() {
    echo -n "Checking Python version... "
    if command -v python3 &> /dev/null; then
        local version=$(python3 --version 2>&1 | awk '{print $2}')
        local major=$(echo "$version" | cut -d. -f1)
        local minor=$(echo "$version" | cut -d. -f2)

        if [ "$major" -ge 3 ] && [ "$minor" -ge 11 ]; then
            echo -e "${GREEN}✓${NC} $version (meets requirement: 3.11+)"
            return 0
        else
            echo -e "${RED}✗${NC} $version (requires 3.11+)"
            echo -e "   Install Python 3.11+ from: https://www.python.org/"
            ((ERRORS++))
            return 1
        fi
    else
        echo -e "${RED}✗${NC} Python 3 not found"
        ((ERRORS++))
        return 1
    fi
}

# Function to test spec-kit access
test_speckit() {
    echo -n "Testing spec-kit CLI access... "
    if uvx --from git+https://github.com/github/spec-kit.git specify --help &> /dev/null; then
        echo -e "${GREEN}✓${NC} Can run spec-kit via uvx"
        return 0
    else
        echo -e "${RED}✗${NC} Cannot run spec-kit"
        echo -e "   Make sure uv/uvx is installed and working"
        ((ERRORS++))
        return 1
    fi
}

# Function to test spec-kit-mcp binary
test_mcp_binary() {
    echo -n "Testing spec-kit-mcp binary... "
    if [ -f "./target/release/spec-kit-mcp" ]; then
        if ./target/release/spec-kit-mcp --version &> /dev/null; then
            local version=$(./target/release/spec-kit-mcp --version)
            echo -e "${GREEN}✓${NC} $version"
            return 0
        else
            echo -e "${RED}✗${NC} Binary exists but fails to run"
            ((ERRORS++))
            return 1
        fi
    else
        echo -e "${YELLOW}⚠${NC} Binary not found (run 'cargo build --release')"
        ((WARNINGS++))
        return 1
    fi
}

echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}1. Required Tools${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

check_python_version
check_command "uv" "uv package manager" "https://docs.astral.sh/uv/" "true"
check_command "uvx" "uvx runner" "https://docs.astral.sh/uv/" "true"
check_command "git" "Git" "https://git-scm.com/" "true"

echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}2. Spec-Kit Access${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

test_speckit

echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}3. Development Tools${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

check_command "cargo" "Rust/Cargo" "https://rustup.rs/" "true"
check_command "rustc" "Rust compiler" "https://rustup.rs/" "true"
check_command "node" "Node.js" "https://nodejs.org/" "false"
check_command "npm" "npm" "https://nodejs.org/" "false"

echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}4. MCP Server Binary${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

test_mcp_binary

echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}5. AI Coding Assistants${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

check_command "code" "VS Code" "https://code.visualstudio.com/" "false"
check_command "cursor" "Cursor" "https://cursor.sh/" "false"
check_command "windsurf" "Windsurf" "https://codeium.com/windsurf" "false"

echo ""
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${BLUE}Summary${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

if [ $ERRORS -eq 0 ]; then
    echo -e "${GREEN}✓ All required tools are installed!${NC}"
    echo ""
    echo -e "You can now:"
    echo -e "  1. Build the project: ${BLUE}cargo build --release${NC}"
    echo -e "  2. Run tests: ${BLUE}cargo test${NC}"
    echo -e "  3. Install the binary: ${BLUE}cargo install --path .${NC}"
    echo ""
    if [ $WARNINGS -gt 0 ]; then
        echo -e "${YELLOW}Note: $WARNINGS optional tool(s) missing${NC}"
    fi
    exit 0
else
    echo -e "${RED}✗ $ERRORS required tool(s) missing${NC}"
    echo ""
    echo -e "Please install the missing tools and run this script again."
    exit 1
fi
