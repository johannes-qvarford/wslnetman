# WSLNetMan Fix Script (PowerShell)
# Formats code and applies Clippy suggestions

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Write-Info($msg) { Write-Host $msg -ForegroundColor Yellow }
function Write-Success($msg) { Write-Host $msg -ForegroundColor Green }
function Write-ErrorMsg($msg) { Write-Host $msg -ForegroundColor Red }

try {
    # Ensure cargo is available
    $null = Get-Command cargo -ErrorAction Stop
} catch {
    Write-ErrorMsg "Error: 'cargo' is not installed or not in PATH."
    exit 1
}

Write-Info 'Running cargo fmt...'
$fmt = Start-Process -FilePath 'cargo' -ArgumentList 'fmt' -NoNewWindow -PassThru -Wait
if ($fmt.ExitCode -ne 0) {
    Write-ErrorMsg "cargo fmt failed with exit code $($fmt.ExitCode)"
    exit $fmt.ExitCode
}

Write-Info 'Running cargo clippy --fix --allow-dirty...'
$clippy = Start-Process -FilePath 'cargo' -ArgumentList @('clippy','--fix','--allow-dirty') -NoNewWindow -PassThru -Wait
if ($clippy.ExitCode -ne 0) {
    Write-ErrorMsg "cargo clippy --fix failed with exit code $($clippy.ExitCode)"
    exit $clippy.ExitCode
}

Write-Success 'Formatting and Clippy fixes completed successfully.'
