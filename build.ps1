# FractureOS Build Script for Windows
# This script helps build and run FractureOS

param(
    [Parameter(Position=0)]
    [ValidateSet('build', 'run', 'test', 'clean', 'check', 'clippy', 'fmt', 'help')]
    [string]$Command = 'help'
)

function Show-Banner {
    Write-Host "╔═══════════════════════════════════════════════════════╗" -ForegroundColor Cyan
    Write-Host "║           FractureOS Build System v0.1.0             ║" -ForegroundColor Cyan
    Write-Host "║     A Modern Rust-based Unix-like Operating System   ║" -ForegroundColor Cyan
    Write-Host "╚═══════════════════════════════════════════════════════╝" -ForegroundColor Cyan
    Write-Host ""
}

function Build-Kernel {
    Write-Host "[BUILD] Building FractureOS kernel..." -ForegroundColor Green
    cargo build --release
    if ($LASTEXITCODE -eq 0) {
        Write-Host "[OK] Build successful!" -ForegroundColor Green
    } else {
        Write-Host "[ERROR] Build failed!" -ForegroundColor Red
        exit 1
    }
}

function Run-Kernel {
    Write-Host "[RUN] Starting FractureOS in QEMU..." -ForegroundColor Green
    cargo run --release
}

function Test-Kernel {
    Write-Host "[TEST] Running kernel tests..." -ForegroundColor Green
    cargo test
}

function Clean-Build {
    Write-Host "[CLEAN] Cleaning build artifacts..." -ForegroundColor Yellow
    cargo clean
    Write-Host "[OK] Clean complete!" -ForegroundColor Green
}

function Check-Code {
    Write-Host "[CHECK] Checking code..." -ForegroundColor Green
    cargo check
}

function Run-Clippy {
    Write-Host "[CLIPPY] Running clippy linter..." -ForegroundColor Green
    cargo clippy
}

function Format-Code {
    Write-Host "[FMT] Formatting code..." -ForegroundColor Green
    cargo fmt
}

function Show-Help {
    Write-Host "FractureOS Build System" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Usage: .\build.ps1 <command>" -ForegroundColor White
    Write-Host ""
    Write-Host "Available commands:" -ForegroundColor Yellow
    Write-Host "  build   - Build the kernel" -ForegroundColor White
    Write-Host "  run     - Run in QEMU" -ForegroundColor White
    Write-Host "  test    - Run tests" -ForegroundColor White
    Write-Host "  clean   - Clean build artifacts" -ForegroundColor White
    Write-Host "  check   - Check code without building" -ForegroundColor White
    Write-Host "  clippy  - Run clippy linter" -ForegroundColor White
    Write-Host "  fmt     - Format code" -ForegroundColor White
    Write-Host "  help    - Show this help message" -ForegroundColor White
    Write-Host ""
    Write-Host "Examples:" -ForegroundColor Yellow
    Write-Host "  .\build.ps1 build" -ForegroundColor Gray
    Write-Host "  .\build.ps1 run" -ForegroundColor Gray
    Write-Host "  .\build.ps1 test" -ForegroundColor Gray
}

# Main execution
Show-Banner

switch ($Command) {
    'build'  { Build-Kernel }
    'run'    { Run-Kernel }
    'test'   { Test-Kernel }
    'clean'  { Clean-Build }
    'check'  { Check-Code }
    'clippy' { Run-Clippy }
    'fmt'    { Format-Code }
    'help'   { Show-Help }
    default  { Show-Help }
}
