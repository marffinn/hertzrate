#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gui;
mod monitor;

use anyhow::Result;

fn main() -> Result<()> {
    gui::run_gui()
}
