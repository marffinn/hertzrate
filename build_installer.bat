@echo off
REM HertzRate Installer Builder (Batch version)
REM This script builds the Windows installer for HertzRate

echo Building HertzRate Windows Installer...
echo.

REM Check if Inno Setup is installed
set INNO_PATH=""
if exist "%ProgramFiles(x86)%\Inno Setup 6\ISCC.exe" set INNO_PATH="%ProgramFiles(x86)%\Inno Setup 6\ISCC.exe"
if exist "%ProgramFiles%\Inno Setup 6\ISCC.exe" set INNO_PATH="%ProgramFiles%\Inno Setup 6\ISCC.exe"
if exist "%ProgramFiles(x86)%\Inno Setup 5\ISCC.exe" set INNO_PATH="%ProgramFiles(x86)%\Inno Setup 5\ISCC.exe"
if exist "%ProgramFiles%\Inno Setup 5\ISCC.exe" set INNO_PATH="%ProgramFiles%\Inno Setup 5\ISCC.exe"

if %INNO_PATH%=="" (
    echo ERROR: Inno Setup not found!
    echo Please install Inno Setup from https://jrsoftware.org/isinfo.php
    echo After installation, run this script again.
    pause
    exit /b 1
)

echo Found Inno Setup at: %INNO_PATH%

REM Build the Rust application
echo Building Rust application...
cargo build --release
if errorlevel 1 (
    echo ERROR: Failed to build Rust application
    pause
    exit /b 1
)
echo ✓ Rust build completed successfully

REM Check if executables exist
if not exist "target\release\hertzrate-gui.exe" (
    echo ERROR: GUI executable not found
    pause
    exit /b 1
)

if not exist "target\release\hertzrate.exe" (
    echo ERROR: CLI executable not found
    pause
    exit /b 1
)

echo ✓ Executables found

REM Create installer directory
if not exist "installer" mkdir installer
echo ✓ Installer directory ready

REM Build the installer
echo Building installer with Inno Setup...
%INNO_PATH% "installer.iss"
if errorlevel 1 (
    echo ERROR: Failed to build installer
    pause
    exit /b 1
)

echo.
echo 🎉 Installer created successfully!
echo Check the 'installer' folder for the setup file.
echo.
pause
