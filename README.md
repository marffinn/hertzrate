# HertzRate - Monitor Refresh Rate Manager

A Rust application to manage refresh rates for all connected monitors on Windows, available with both GUI and CLI interfaces.

## Features

- 🖥️ **Data-Adaptive GUI** - Intelligent interface that adapts to your monitor setup and window size
- 📋 **List all connected monitors** with their current settings and available refresh rates
- ⚡ **Set refresh rate for specific monitor** by index or through GUI dropdowns

- 📊 **Display current resolution and refresh rate** for each monitor
- ✅ **Error handling** with detailed feedback
- 🔄 **Auto-refresh** - GUI automatically updates monitor information every 5 seconds
- 💻 **Command-line interface** - Full CLI support for automation and scripting

## Installation

### Prerequisites
- Windows 10/11
- Rust toolchain (install from [rustup.rs](https://rustup.rs/))

### Build from source
```bash
git clone <repository-url>
cd hertzrate
cargo build --release
```

Two executables will be available:
- `target/release/hertzrate-gui.exe` - GUI version (no console window)
- `target/release/hertzrate.exe` - CLI version

### Quick Usage
```bash
# Launch GUI (no console window)
.\target\release\hertzrate-gui.exe

# Using launcher scripts (defaults to GUI)
hertzrate.bat
.\hertzrate.ps1

# CLI commands
.\target\release\hertzrate.exe list
.\target\release\hertzrate.exe set -m 0 -r 144

# Using launcher scripts for CLI
hertzrate.bat list
.\hertzrate.ps1 set -m 0 -r 144
```

## Usage

### GUI Mode

Launch the GUI version which runs without a console window:
```bash
.\target\release\hertzrate-gui.exe
```

The GUI provides:
- **Data-Adaptive Design**: Automatically adjusts layout based on number of monitors and available space
- **Multiple Layout Modes**:
  - Grid layout for multiple monitors on wide screens
  - Compact layout for narrow windows
  - Minimal layout for many monitors in limited space
  - Standard layout for optimal viewing
- **Smart Window Sizing**: Initial size adapts to detected monitor count
- **Intelligent Status Display**: Shows refresh rate conflicts and monitor diversity
- **Monitor Cards**: Truncated text and optimized spacing based on content
- **Apply Buttons**: Apply changes to individual monitors
- **Real-time Feedback**: Status messages and auto-refresh every 5 seconds
- **Fully Resizable**: Minimum 400×300, maximum 1200×800 pixels

### CLI Mode

#### List all monitors
```bash
hertzrate list
```

This will show all connected monitors with:
- Monitor index (for use with other commands)
- Monitor description/name
- Device name
- Current resolution
- Current refresh rate
- All available refresh rates

Example output:
```
Connected Monitors:
==================================================
Monitor 0: Generic PnP Monitor
  Device: \\.\DISPLAY1
  Resolution: 1920x1080
  Current Refresh Rate: 60Hz
  Available Refresh Rates: [60, 75, 120, 144]Hz

Monitor 1: ASUS VG248QE
  Device: \\.\DISPLAY2
  Resolution: 1920x1080
  Current Refresh Rate: 144Hz
  Available Refresh Rates: [60, 75, 120, 144]Hz
```

#### Set refresh rate for a specific monitor
```bash
hertzrate set --monitor 0 --rate 144
```

This sets the refresh rate to 144Hz for monitor 0 (first monitor).

#### Set refresh rate for all monitors
```bash
hertzrate set-all --rate 60
```

This attempts to set all monitors to 60Hz. If a monitor doesn't support the specified refresh rate, it will show an error but continue with other monitors.

## Command Reference

### `hertzrate list`
Lists all connected monitors and their capabilities.

### `hertzrate set -m <INDEX> -r <RATE>`
Sets refresh rate for a specific monitor.
- `-m, --monitor <INDEX>`: Monitor index (0-based, use `list` to see available monitors)
- `-r, --rate <RATE>`: Refresh rate in Hz

### `hertzrate set-all -r <RATE>`
Sets refresh rate for all monitors.
- `-r, --rate <RATE>`: Refresh rate in Hz

### `hertzrate --help`
Shows help information and available commands.

## Examples

```bash
# List all monitors
hertzrate list

# Set laptop screen (monitor 0) to 60Hz
hertzrate set -m 0 -r 60

# Set external monitor (monitor 1) to 144Hz
hertzrate set -m 1 -r 144

# Set all monitors to 75Hz
hertzrate set-all -r 75
```

## Troubleshooting

### "No monitors found"
- Ensure your monitors are properly connected and recognized by Windows
- Check Windows Display Settings to verify monitors are detected

### "Refresh rate XHz is not available"
- Use `hertzrate list` to see available refresh rates for each monitor
- Some monitors may not support certain refresh rates at their current resolution

### "Failed to change refresh rate"
- Try running as Administrator
- Ensure the monitor supports the requested refresh rate
- Check that no applications are preventing display changes

### Permission Issues
Some display changes may require administrator privileges. Try running the command prompt as Administrator.

## Technical Details

This tool uses the Windows GDI API functions:
- `EnumDisplayDevices` - to enumerate connected monitors
- `EnumDisplaySettings` - to get current and available display modes
- `ChangeDisplaySettings` - to change the refresh rate

The tool preserves the current resolution and only changes the refresh rate.

## License

MIT License - see LICENSE file for details.
