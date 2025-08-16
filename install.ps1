# WSLNetMan Install Script (PowerShell)
# Sets up a pre-commit hook for code quality checks

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Write-Info($msg) { Write-Host $msg -ForegroundColor Yellow }
function Write-Success($msg) { Write-Host $msg -ForegroundColor Green }
function Write-ErrorMsg($msg) { Write-Host $msg -ForegroundColor Red }

Write-Success "WSLNetMan Install Script"
Write-Info "Setting up pre-commit hook..."

# Ensure we're at the repository root (has .git)
if (-not (Test-Path -Path '.git')) {
    Write-ErrorMsg "Error: This script must be run from the root of the git repository"
    exit 1
}

# Ensure hooks directory exists
$hooksDir = Join-Path '.git' 'hooks'
New-Item -ItemType Directory -Path $hooksDir -Force | Out-Null

# Content of the pre-commit hook (Bash script run by Git for Windows via sh.exe)
$preCommitContent = @'
#!/bin/bash

set -e

echo "Running pre-commit checks..."

# Check if cargo is installed
if ! command -v cargo >/dev/null 2>&1; then
    echo "Error: cargo is not installed or not in PATH"
    exit 1
fi

# Check code formatting
echo "Checking code formatting..."
if ! cargo fmt -- --check; then
    echo "Error: Code formatting check failed"
    echo "Run 'cargo fmt' to fix formatting issues"
    exit 1
fi

# Run Clippy linting
echo "Running Clippy linting..."
if ! cargo clippy -- -D warnings; then
    echo "Error: Clippy check failed"
    exit 1
fi


echo "All pre-commit checks passed!"
'@

$preCommitPath = Join-Path $hooksDir 'pre-commit'
Set-Content -Path $preCommitPath -Value $preCommitContent -Encoding UTF8

# On Windows, Git does not require chmod +x; but set the file attribute to Normal just in case
try {
    attrib -R $preCommitPath | Out-Null
} catch { }

Write-Success "Pre-commit hook installed successfully!"
Write-Info "The hook will automatically run:"
Write-Info "  - cargo fmt -- --check"
Write-Info "  - cargo clippy -- -D warnings"
Write-Host ''
Write-Success 'Happy coding!'
