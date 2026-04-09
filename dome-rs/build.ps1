# Build script for DOME on Windows PowerShell
# Run with: powershell -ExecutionPolicy Bypass -File build.ps1

Write-Host "DOME Build Script for Windows" -ForegroundColor Cyan
Write-Host "=================================" -ForegroundColor Cyan
Write-Host ""

# Check if Rust is installed
$rustInstalled = $false
try {
    $rustVersion = rustc --version 2>$null
    Write-Host "Rust is installed:" -ForegroundColor Green
    Write-Host "  $rustVersion" -ForegroundColor Green
    $rustInstalled = $true
} catch {
    Write-Host "Rust not found. Installing from https://rustup.rs/" -ForegroundColor Red
    Write-Host ""
    
    # Download and run rustup
    $rustupUrl = "https://win.rustup.rs/x86_64"
    $rustupPath = "$env:TEMP\rustup-init.exe"
    
    Write-Host "Downloading rustup..." -ForegroundColor Yellow
    Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupPath
    
    Write-Host "Running installer..." -ForegroundColor Yellow
    & $rustupPath -y
    
    $env:Path += ";$env:USERPROFILE\.cargo\bin"
    
    Write-Host "Rust installed successfully" -ForegroundColor Green
    Write-Host ""
}

Write-Host "Rust version:" -ForegroundColor Cyan
cargo --version
rustc --version

# Check for MinGW-w64 and dlltool
Write-Host ""
Write-Host "Checking build tools..." -ForegroundColor Cyan

$hasMinGW = $false
try {
    $dlltoolVersion = dlltool --version 2>$null
    if ($LASTEXITCODE -eq 0) {
        $hasMinGW = $true
        Write-Host "MinGW-w64 found: $($dlltoolVersion.Split([Environment]::NewLine)[0])" -ForegroundColor Green
    }
} catch {
    $hasMinGW = $false
}

if (-not $hasMinGW) {
    Write-Host "MinGW-w64 not found. Installing..." -ForegroundColor Yellow
    
    # Download MinGW-w64
    $mingwUrl = "https://github.com/niXman/mingw-builds-binaries/releases/download/14.2.0-rt_v12-rev0/x86_64-14.2.0-release-posix-seh-ucrt-rt_v12-rev0.7z"
    $mingwPath = "$env:TEMP\mingw-w64.7z"
    $mingwInstallPath = "C:\MinGW-w64"
    
    Write-Host "Downloading MinGW-w64 (~80MB)..." -ForegroundColor Yellow
    $ProgressPreference = 'SilentlyContinue'
    Invoke-WebRequest -Uri $mingwUrl -OutFile $mingwPath
    $ProgressPreference = 'Continue'
    
    Write-Host "Extracting MinGW-w64..." -ForegroundColor Yellow
    if (-not (Test-Path "C:\Program Files\7-Zip\7z.exe")) {
        Write-Host "7-Zip not found. Please extract manually:" -ForegroundColor Red
        Write-Host "  $mingwPath -> $mingwInstallPath" -ForegroundColor Yellow
        exit 1
    }
    
    & "C:\Program Files\7-Zip\7z.exe" x $mingwPath -o$mingwInstallPath -y | Out-Null
    
    Write-Host "Adding MinGW to PATH..." -ForegroundColor Yellow
    $mingwBin = "$mingwInstallPath\mingw64\bin"
    [Environment]::SetEnvironmentVariable('PATH', "$mingwBin;$([Environment]::GetEnvironmentVariable('PATH','User'))", 'User')
    $env:PATH = "$mingwBin;$env:PATH"
    
    Write-Host "MinGW-w64 installed successfully" -ForegroundColor Green
}

# Configure Rust for GNU toolchain
Write-Host "Configuring Rust GNU toolchain..." -ForegroundColor Cyan
rustup target install x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
Write-Host "Rust GNU toolchain configured" -ForegroundColor Green

# Navigate to dome-rs directory
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

Write-Host ""
Write-Host "Building DOME (this may take a minute)..." -ForegroundColor Cyan
Write-Host ""

# Run cargo build
cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "Build successful!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Executable location:" -ForegroundColor Cyan
    Write-Host "   $scriptDir\target\release\dome.exe" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "To run DOME:" -ForegroundColor Cyan
    Write-Host "   .\target\release\dome.exe" -ForegroundColor Green
    Write-Host ""
    Write-Host "Quick start:" -ForegroundColor Cyan
    Write-Host "   Type 'help' inside vault for available commands" -ForegroundColor Green
} else {
    Write-Host ""
    Write-Host "Build failed. See errors above." -ForegroundColor Red
    exit 1
}
