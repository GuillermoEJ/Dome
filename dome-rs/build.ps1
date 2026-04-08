# Build script for DOME on Windows PowerShell
# Run with: powershell -ExecutionPolicy Bypass -File build.ps1

Write-Host "🚀 DOME Build Script for Windows" -ForegroundColor Cyan
Write-Host "=================================" -ForegroundColor Cyan
Write-Host ""

# Check if Rust is installed
try {
    $rustVersion = rustc --version 2>$null
    Write-Host "✓ Rust is installed:" -ForegroundColor Green
    Write-Host "  $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "❌ Rust not found. Installing from https://rustup.rs/" -ForegroundColor Red
    Write-Host ""
    
    # Download and run rustup
    $rustupUrl = "https://win.rustup.rs/x86_64"
    $rustupPath = "$env:TEMP\rustup-init.exe"
    
    Write-Host "Downloading rustup..." -ForegroundColor Yellow
    Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupPath
    
    Write-Host "Running installer..." -ForegroundColor Yellow
    & $rustupPath -y
    
    $env:Path += ";$env:USERPROFILE\.cargo\bin"
    
    Write-Host "✓ Rust installed successfully" -ForegroundColor Green
    Write-Host ""
}

Write-Host "Rust version:" -ForegroundColor Cyan
cargo --version
rustc --version

# Navigate to dome-rs directory
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location "$scriptDir\dome-rs"

Write-Host ""
Write-Host "📦 Building DOME (this may take a minute)..." -ForegroundColor Cyan
Write-Host ""

# Run cargo build
cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "✅ Build successful!" -ForegroundColor Green
    Write-Host ""
    Write-Host "📍 Executable location:" -ForegroundColor Cyan
    Write-Host "   $(Get-Location)\target\release\dome.exe" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "🎯 To run DOME:" -ForegroundColor Cyan
    Write-Host "   .\target\release\dome.exe" -ForegroundColor Green
    Write-Host ""
    Write-Host "📚 Quick start:" -ForegroundColor Cyan
    Write-Host "   Type 'help' inside vault for available commands" -ForegroundColor Green
} else {
    Write-Host ""
    Write-Host "❌ Build failed. See errors above." -ForegroundColor Red
    exit 1
}
