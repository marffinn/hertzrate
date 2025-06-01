mod gui;
mod monitor;

use anyhow::Result;
use clap::{Parser, Subcommand};
use monitor::Monitor;

#[derive(Parser)]
#[command(name = "hertzrate")]
#[command(about = "A tool to manage monitor refresh rates")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch the graphical user interface
    Gui,
    /// List all connected monitors and their available refresh rates
    List,
    /// Set refresh rate for a specific monitor
    Set {
        /// Monitor index (use 'list' command to see available monitors)
        #[arg(short, long)]
        monitor: usize,
        /// Refresh rate in Hz
        #[arg(short, long)]
        rate: u32,
    },
    /// Set refresh rate for all monitors
    SetAll {
        /// Refresh rate in Hz
        #[arg(short, long)]
        rate: u32,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Gui) => gui::run_gui()?,
        Some(Commands::List) => list_monitors()?,
        Some(Commands::Set { monitor, rate }) => set_monitor_refresh_rate(monitor, rate)?,
        Some(Commands::SetAll { rate }) => set_all_monitors_refresh_rate(rate)?,
        None => {
            // If no command specified, show help and suggest using GUI
            println!("HertzRate - Monitor Refresh Rate Manager");
            println!();
            println!("For GUI mode, run: hertzrate-gui.exe");
            println!("For CLI help, run: hertzrate.exe --help");
            println!();
            list_monitors()?;
        }
    }

    Ok(())
}

fn list_monitors() -> Result<()> {
    let monitors = Monitor::enumerate_monitors()?;

    if monitors.is_empty() {
        println!("No monitors found.");
        return Ok(());
    }

    println!("Connected Monitors:");
    println!("{}", "=".repeat(50));

    for (index, monitor) in monitors.iter().enumerate() {
        println!("Monitor {}: {}", index, monitor.description);
        println!("  Device: {}", monitor.device_name);
        println!(
            "  Resolution: {}x{}",
            monitor.current_width, monitor.current_height
        );
        println!("  Current Refresh Rate: {}Hz", monitor.current_refresh_rate);
        println!(
            "  Available Refresh Rates: {:?}Hz",
            monitor.available_refresh_rates
        );
        println!();
    }

    Ok(())
}

fn set_monitor_refresh_rate(monitor_index: usize, refresh_rate: u32) -> Result<()> {
    let monitors = Monitor::enumerate_monitors()?;

    if monitor_index >= monitors.len() {
        return Err(anyhow::anyhow!(
            "Monitor index {} is out of range. Available monitors: 0-{}",
            monitor_index,
            monitors.len() - 1
        ));
    }

    let monitor = &monitors[monitor_index];
    println!(
        "Setting refresh rate to {}Hz for monitor: {}",
        refresh_rate, monitor.description
    );

    monitor.set_refresh_rate(refresh_rate)?;

    println!("✓ Successfully changed refresh rate to {}Hz", refresh_rate);
    Ok(())
}

fn set_all_monitors_refresh_rate(refresh_rate: u32) -> Result<()> {
    let monitors = Monitor::enumerate_monitors()?;

    if monitors.is_empty() {
        println!("No monitors found.");
        return Ok(());
    }

    println!(
        "Setting refresh rate to {}Hz for all monitors...",
        refresh_rate
    );

    let mut success_count = 0;
    let mut errors = Vec::new();

    for (index, monitor) in monitors.iter().enumerate() {
        match monitor.set_refresh_rate(refresh_rate) {
            Ok(()) => {
                println!("✓ Monitor {}: {} - Success", index, monitor.description);
                success_count += 1;
            }
            Err(e) => {
                println!(
                    "✗ Monitor {}: {} - Failed: {}",
                    index, monitor.description, e
                );
                errors.push((index, e));
            }
        }
    }

    println!();
    println!(
        "Summary: {}/{} monitors updated successfully",
        success_count,
        monitors.len()
    );

    if !errors.is_empty() {
        println!("Errors occurred for {} monitor(s):", errors.len());
        for (index, error) in errors {
            println!("  Monitor {}: {}", index, error);
        }
    }

    Ok(())
}
