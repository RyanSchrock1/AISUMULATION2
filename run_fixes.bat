@echo off
REM run_fixes.bat - Run all lint fixing scripts

echo === Starting Lint Fixing Process ===

REM Install required tools
echo.
echo === Installing required tools ===
rustup component add rustfmt
rustup component add clippy

REM Install cargo-edit if not installed
where cargo-add >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo Installing cargo-edit...
    cargo install cargo-edit
)

REM Build the scripts
echo.
echo === Building scripts ===
cd scripts
cargo build
cd ..

REM Run the linter to analyze issues
echo.
echo === Analyzing lint issues ===
scripts\target\debug\analyze_lints.exe || echo Analysis completed with warnings

REM Run the fix scripts
echo.
echo === Running fix_common_issues ===
scripts\target\debug\fix_common_issues.exe || echo Fixes completed with warnings

echo.
echo === Running fix_crate_paths ===
scripts\target\debug\fix_crate_paths.exe || echo Crate path fixes completed with warnings

echo.
echo === Running fix_clone_boxed ===
scripts\target\debug\fix_clone_boxed.exe || echo Clone boxed fixes completed with warnings

REM Format all code
echo.
echo === Formatting code ===
for /r src\ %%f in (*.rs) do rustfmt "%%f"

REM Run clippy with auto-fix
echo.
echo === Running clippy with auto-fix ===
cargo clippy --fix --allow-dirty --allow-staged || echo Clippy completed with warnings

REM Final check
echo.
echo === Final lint check ===
cargo check

echo.
echo === Lint fixing process completed ===
echo Please review the changes and commit them to version control.

pause
