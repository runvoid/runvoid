# ==============================================================================
# Runvoid Windows x86_64 PowerShell Installer
# Supported Architecture: Windows x64 (AMD64)
# ==============================================================================

[CmdletBinding()]
param (
    [switch]$Uninstall,
    [string]$InstallPath = "$env:LOCALAPPDATA\Programs\Runvoid"
)

$ErrorActionPreference = "Stop"

Write-Host "╔══════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║             🚀 Runvoid Programming Language Installer         ║" -ForegroundColor Cyan
Write-Host "║                   Target: Windows x86_64                     ║" -ForegroundColor Cyan
Write-Host "╚══════════════════════════════════════════════════════════════╝`n" -ForegroundColor Cyan

$BinDir = Join-Path $InstallPath "bin"
$ExePath = Join-Path $BinDir "runvoid.exe"

# Handle Uninstall
if ($Uninstall) {
    Write-Host "Uninstalling Runvoid..." -ForegroundColor Yellow
    if (Test-Path $InstallPath) {
        Remove-Item -Recurse -Force $InstallPath
        Write-Host "✓ Removed $InstallPath" -ForegroundColor Green
    }
    
    # Remove from User PATH
    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($UserPath -like "*$BinDir*") {
        $NewPath = ($UserPath.Split(';') | Where-Object { $_ -ne $BinDir -and $_ -ne "" }) -join ';'
        [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
        Write-Host "✓ Removed Runvoid from User PATH" -ForegroundColor Green
    }
    
    Write-Host "Runvoid has been successfully uninstalled from your system." -ForegroundColor Green
    exit 0
}

# 1. Architecture Check
if (-not [Environment]::Is64BitOperatingSystem) {
    Write-Error "Error: Runvoid requires a 64-bit (x86_64) Windows operating system."
    exit 1
}
Write-Host "✓ 64-bit Windows operating system detected." -ForegroundColor Green

# 2. Check Dependencies
Write-Host "`nChecking required tools..." -ForegroundColor Blue
$HasNasm = Get-Command "nasm.exe" -ErrorAction SilentlyContinue
$HasGcc = Get-Command "gcc.exe" -ErrorAction SilentlyContinue
$HasCargo = Get-Command "cargo.exe" -ErrorAction SilentlyContinue

if (-not $HasNasm) {
    Write-Host "⚠ Note: NASM (nasm.exe) was not found in PATH." -ForegroundColor Yellow
    Write-Host "  You can install it via Winget: winget install -e --id NASM.NASM" -ForegroundColor Yellow
    Write-Host "  Or via Chocolatey: choco install nasm" -ForegroundColor Yellow
} else {
    Write-Host "✓ Found NASM assembler: $($HasNasm.Source)" -ForegroundColor Green
}

if (-not $HasGcc) {
    Write-Host "⚠ Note: GCC (MinGW-w64) was not found in PATH." -ForegroundColor Yellow
    Write-Host "  You can install it via Winget: winget install -e --id MSYS2.MSYS2 or winlibs" -ForegroundColor Yellow
} else {
    Write-Host "✓ Found GCC linker: $($HasGcc.Source)" -ForegroundColor Green
}

# 3. Build or Locate Binary
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$SourceBin = Join-Path $ScriptDir "target\release\runvoid.exe"

if (-not (Test-Path $SourceBin) -and $HasCargo) {
    Write-Host "`nCompiling Runvoid from source using cargo..." -ForegroundColor Blue
    Push-Location $ScriptDir
    try {
        cargo build --release
    } finally {
        Pop-Location
    }
}

if (-not (Test-Path $SourceBin)) {
    # Check if a prebuilt binary or debug binary exists
    $DebugBin = Join-Path $ScriptDir "target\debug\runvoid.exe"
    if (Test-Path $DebugBin) {
        $SourceBin = $DebugBin
    } else {
        Write-Error "Could not find compiled binary at '$SourceBin'. Please build with 'cargo build --release' first."
        exit 1
    }
}

# 4. Install Binary
Write-Host "`nInstalling Runvoid to: $BinDir" -ForegroundColor Blue
if (-not (Test-Path $BinDir)) {
    New-Item -ItemType Directory -Force -Path $BinDir | Out-Null
}

Copy-Item -Path $SourceBin -Destination $ExePath -Force
Write-Host "✓ Installed executable: $ExePath" -ForegroundColor Green

# 5. Add to User PATH
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$BinDir*") {
    $NewPath = if ([string]::IsNullOrWhiteSpace($UserPath)) { $BinDir } else { "$UserPath;$BinDir" }
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    $env:Path = "$env:Path;$BinDir"
    Write-Host "✓ Added $BinDir to User PATH environment variable." -ForegroundColor Green
} else {
    Write-Host "✓ $BinDir is already in User PATH." -ForegroundColor Green
}

# 6. Verify Installation
Write-Host "`nVerifying installation:" -ForegroundColor Cyan
try {
    & $ExePath --version
} catch {
    Write-Host "Installed version checked."
}

Write-Host "`n🎉 Runvoid is ready to use on Windows!" -ForegroundColor Green
Write-Host "Open a new terminal window and try:"
Write-Host "  runvoid cheat       - View interactive syntax cheat sheet"
Write-Host "  runvoid repl        - Launch interactive coding shell"
Write-Host "  runvoid new game    - Create starter game project"
Write-Host "  runvoid test        - Run automated tests"
