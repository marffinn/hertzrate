# HertzRate Windows Installer

This directory contains the files needed to create a Windows installer for HertzRate.

## Quick Start

### Option 1: PowerShell (Recommended)
```powershell
.\build_installer.ps1
```

### Option 2: Batch File
```cmd
build_installer.bat
```

## Prerequisites

1. **Rust Toolchain**: Install from [rustup.rs](https://rustup.rs/)
2. **Inno Setup**: Download and install from [jrsoftware.org/isinfo.php](https://jrsoftware.org/isinfo.php)

## What the Installer Includes

- **HertzRate GUI** (`hertzrate-gui.exe`) - Main graphical application
- **HertzRate CLI** (`hertzrate.exe`) - Command-line interface
- **Documentation** (`README.md`)
- **License** (`LICENSE`)

## Installation Features

### For End Users:
- **Easy Installation**: Standard Windows installer experience
- **Desktop Shortcut**: Optional desktop icon for quick access
- **Start Menu Integration**: Adds HertzRate to Start Menu
- **PATH Integration**: Optional CLI access from anywhere
- **Clean Uninstall**: Complete removal through Windows Programs & Features

### Installation Options:
- **Desktop Icon**: Creates desktop shortcut (optional)
- **Quick Launch**: Adds to Quick Launch bar (Windows 7 and below)
- **Add to PATH**: Enables CLI usage from any command prompt (optional)

## Installer Details

- **Size**: ~5-8 MB (compressed)
- **Target**: Windows 10/11 (64-bit)
- **Privileges**: Requires Administrator rights for system-wide installation
- **Install Location**: `C:\Program Files\HertzRate\` (default)

## Building the Installer

### Automatic Build (Recommended)
```powershell
# Build everything and create installer
.\build_installer.ps1

# Skip Rust build if already built
.\build_installer.ps1 -SkipBuild

# Show help
.\build_installer.ps1 -Help
```

### Manual Build Steps
1. Build the Rust application:
   ```bash
   cargo build --release
   ```

2. Ensure Inno Setup is installed

3. Run Inno Setup compiler:
   ```cmd
   "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" installer.iss
   ```

4. Find the installer in the `installer/` directory

## Customization

### Modify `installer.iss` to:
- Change app version
- Update publisher information
- Modify installation directory
- Add/remove components
- Change installer appearance

### Key Sections:
- `[Setup]`: Basic installer configuration
- `[Files]`: Files to include in installation
- `[Icons]`: Start menu and desktop shortcuts
- `[Registry]`: PATH environment variable (optional)
- `[Tasks]`: User-selectable installation options

## Distribution

After building, you'll get:
- `installer/HertzRate-Setup-v0.1.0.exe` - The installer file

This single file can be distributed to install HertzRate on any Windows 10/11 system.

## Troubleshooting

### "Inno Setup not found"
- Download and install Inno Setup from the official website
- Ensure it's installed in the default location

### "Executables not found"
- Run `cargo build --release` first
- Check that `target/release/` contains both `.exe` files

### "Permission denied"
- Run PowerShell/Command Prompt as Administrator
- Ensure antivirus isn't blocking the build process

### "Installer won't run"
- The installer requires Administrator privileges
- Right-click the installer and select "Run as administrator"

## Advanced Usage

### Silent Installation
```cmd
HertzRate-Setup-v0.1.0.exe /SILENT
```

### Very Silent Installation (no UI)
```cmd
HertzRate-Setup-v0.1.0.exe /VERYSILENT
```

### Custom Install Directory
```cmd
HertzRate-Setup-v0.1.0.exe /DIR="C:\MyApps\HertzRate"
```

### Skip Desktop Icon
```cmd
HertzRate-Setup-v0.1.0.exe /TASKS="!desktopicon"
```

## Files Included

- `installer.iss` - Inno Setup script
- `build_installer.ps1` - PowerShell build script
- `build_installer.bat` - Batch build script
- `INSTALLER_README.md` - This documentation

## License

The installer scripts are provided under the same MIT license as HertzRate.
