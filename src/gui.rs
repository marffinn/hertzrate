use crate::monitor::Monitor;
use anyhow::Result;
use eframe::egui;
use std::collections::HashMap;

pub struct HertzRateApp {
    monitors: Vec<Monitor>,
    selected_rates: HashMap<usize, u32>,
    status_message: String,
    show_error: bool,
    error_message: String,
    last_refresh: std::time::Instant,
}

impl Default for HertzRateApp {
    fn default() -> Self {
        Self::new()
    }
}

impl HertzRateApp {
    pub fn new() -> Self {
        let mut app = Self {
            monitors: Vec::new(),
            selected_rates: HashMap::new(),
            status_message: String::new(),
            show_error: false,
            error_message: String::new(),
            last_refresh: std::time::Instant::now(),
        };
        app.refresh_monitors();
        app
    }

    fn refresh_monitors(&mut self) {
        match Monitor::enumerate_monitors() {
            Ok(monitors) => {
                // Initialize selected rates with current rates
                for (index, monitor) in monitors.iter().enumerate() {
                    self.selected_rates
                        .insert(index, monitor.current_refresh_rate);
                }
                self.monitors = monitors;
                self.status_message = format!("Found {} monitor(s)", self.monitors.len());
                self.show_error = false;
            }
            Err(e) => {
                self.error_message = format!("Failed to enumerate monitors: {}", e);
                self.show_error = true;
                self.monitors.clear();
            }
        }
    }

    fn apply_rate_change(&mut self, monitor_index: usize) {
        if let Some(&rate) = self.selected_rates.get(&monitor_index) {
            if let Some(monitor) = self.monitors.get(monitor_index) {
                match monitor.set_refresh_rate(rate) {
                    Ok(()) => {
                        self.status_message =
                            format!("✓ Successfully set {} to {}Hz", monitor.description, rate);
                        self.show_error = false;
                        // Refresh monitors to get updated current rates
                        self.refresh_monitors();
                    }
                    Err(e) => {
                        self.error_message = format!(
                            "Failed to set refresh rate for {}: {}",
                            monitor.description, e
                        );
                        self.show_error = true;
                    }
                }
            }
        }
    }

    fn render_monitor_card_grid(
        &self,
        ui: &mut egui::Ui,
        index: usize,
        monitor: &Monitor,
        actions: &mut Vec<usize>,
        rate_changes: &mut Vec<(usize, u32)>,
    ) {
        ui.group(|ui| {
            ui.set_min_width(200.0);
            ui.vertical(|ui| {
                // Compact header for grid
                ui.strong(format!("Monitor {}", index));
                ui.small(self.truncate_text(&monitor.description, 25));

                // Essential info only
                ui.horizontal(|ui| {
                    ui.small(format!(
                        "{}×{}",
                        monitor.current_width, monitor.current_height
                    ));
                    ui.separator();
                    ui.small(format!("{}Hz", monitor.current_refresh_rate));
                });

                ui.add_space(4.0);

                // Compact controls
                let mut selected_rate = self
                    .selected_rates
                    .get(&index)
                    .copied()
                    .unwrap_or(monitor.current_refresh_rate);

                let mut rate_changed = false;
                egui::ComboBox::from_id_source(format!("grid_combo_{}", index))
                    .selected_text(format!("{}Hz", selected_rate))
                    .width(70.0)
                    .show_ui(ui, |ui| {
                        for &rate in &monitor.available_refresh_rates {
                            let text = if rate == monitor.current_refresh_rate {
                                format!("{}Hz ✓", rate)
                            } else {
                                format!("{}Hz", rate)
                            };

                            if ui
                                .selectable_value(&mut selected_rate, rate, text)
                                .changed()
                            {
                                rate_changed = true;
                            }
                        }
                    });

                if rate_changed {
                    rate_changes.push((index, selected_rate));
                }

                if ui.small_button("Apply").clicked() {
                    actions.push(index);
                }
            });
        });
    }

    fn render_monitor_card_minimal(
        &self,
        ui: &mut egui::Ui,
        index: usize,
        monitor: &Monitor,
        actions: &mut Vec<usize>,
        rate_changes: &mut Vec<(usize, u32)>,
    ) {
        ui.horizontal(|ui| {
            ui.label(format!("Monitor {}", index));
            ui.separator();

            let mut selected_rate = self
                .selected_rates
                .get(&index)
                .copied()
                .unwrap_or(monitor.current_refresh_rate);

            let mut rate_changed = false;
            egui::ComboBox::from_id_source(format!("minimal_combo_{}", index))
                .selected_text(format!("{}Hz", selected_rate))
                .width(60.0)
                .show_ui(ui, |ui| {
                    for &rate in &monitor.available_refresh_rates {
                        if ui
                            .selectable_value(&mut selected_rate, rate, format!("{}Hz", rate))
                            .changed()
                        {
                            rate_changed = true;
                        }
                    }
                });

            if rate_changed {
                rate_changes.push((index, selected_rate));
            }

            if ui.small_button("Apply").clicked() {
                actions.push(index);
            }
        });
    }

    fn render_monitor_card_compact(
        &self,
        ui: &mut egui::Ui,
        index: usize,
        monitor: &Monitor,
        actions: &mut Vec<usize>,
        rate_changes: &mut Vec<(usize, u32)>,
    ) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.strong(format!("📺 Monitor {}", index));
                ui.small(self.truncate_text(&monitor.description, 30));

                ui.horizontal_wrapped(|ui| {
                    ui.label(format!(
                        "{}×{}",
                        monitor.current_width, monitor.current_height
                    ));
                    ui.separator();
                    ui.label(format!("{}Hz", monitor.current_refresh_rate));
                });

                ui.add_space(4.0);

                ui.vertical(|ui| {
                    ui.label("Refresh rate:");

                    ui.horizontal(|ui| {
                        let mut selected_rate = self
                            .selected_rates
                            .get(&index)
                            .copied()
                            .unwrap_or(monitor.current_refresh_rate);

                        let mut rate_changed = false;
                        egui::ComboBox::from_id_source(format!("compact_combo_{}", index))
                            .selected_text(format!("{}Hz", selected_rate))
                            .width(80.0)
                            .show_ui(ui, |ui| {
                                for &rate in &monitor.available_refresh_rates {
                                    let text = if rate == monitor.current_refresh_rate {
                                        format!("{}Hz (current)", rate)
                                    } else {
                                        format!("{}Hz", rate)
                                    };

                                    if ui
                                        .selectable_value(&mut selected_rate, rate, text)
                                        .changed()
                                    {
                                        rate_changed = true;
                                    }
                                }
                            });

                        if rate_changed {
                            rate_changes.push((index, selected_rate));
                        }

                        if ui.button("Apply").clicked() {
                            actions.push(index);
                        }
                    });
                });
            });
        });
    }

    fn render_monitor_card_standard(
        &self,
        ui: &mut egui::Ui,
        index: usize,
        monitor: &Monitor,
        actions: &mut Vec<usize>,
        rate_changes: &mut Vec<(usize, u32)>,
    ) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.strong(format!("📺 Monitor {}: {}", index, monitor.description));
                });

                ui.horizontal_wrapped(|ui| {
                    ui.label(format!(
                        "Resolution: {}×{}",
                        monitor.current_width, monitor.current_height
                    ));
                    ui.separator();
                    ui.label(format!("Current: {}Hz", monitor.current_refresh_rate));
                    ui.separator();
                    ui.label(format!(
                        "Available: {:?}Hz",
                        monitor.available_refresh_rates
                    ));
                });

                ui.add_space(4.0);

                ui.horizontal_wrapped(|ui| {
                    ui.label("Set refresh rate:");

                    let mut selected_rate = self
                        .selected_rates
                        .get(&index)
                        .copied()
                        .unwrap_or(monitor.current_refresh_rate);

                    let mut rate_changed = false;
                    egui::ComboBox::from_id_source(format!("standard_combo_{}", index))
                        .selected_text(format!("{}Hz", selected_rate))
                        .width(120.0)
                        .show_ui(ui, |ui| {
                            for &rate in &monitor.available_refresh_rates {
                                let text = if rate == monitor.current_refresh_rate {
                                    format!("{}Hz (current)", rate)
                                } else {
                                    format!("{}Hz", rate)
                                };

                                if ui
                                    .selectable_value(&mut selected_rate, rate, text)
                                    .changed()
                                {
                                    rate_changed = true;
                                }
                            }
                        });

                    if rate_changed {
                        rate_changes.push((index, selected_rate));
                    }

                    if ui.button("Apply").clicked() {
                        actions.push(index);
                    }
                });
            });
        });
    }

    fn truncate_text(&self, text: &str, max_len: usize) -> String {
        if text.len() <= max_len {
            text.to_string()
        } else {
            format!("{}...", &text[..max_len.saturating_sub(3)])
        }
    }
}

impl eframe::App for HertzRateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Auto-refresh every 5 seconds
        if self.last_refresh.elapsed().as_secs() >= 5 {
            self.refresh_monitors();
            self.last_refresh = std::time::Instant::now();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Responsive header
            ui.vertical_centered(|ui| {
                ui.heading("🖥️ HertzRate");
                ui.small("Monitor Refresh Rate Manager");
            });
            ui.separator();

            // Status bar - responsive layout
            ui.horizontal_wrapped(|ui| {
                ui.label("Status:");
                ui.separator();
                if self.show_error {
                    ui.colored_label(egui::Color32::RED, &self.error_message);
                } else {
                    ui.colored_label(egui::Color32::DARK_GREEN, &self.status_message);
                }
            });

            ui.separator();

            // Top controls - data-adaptive layout
            ui.horizontal_wrapped(|ui| {
                if ui.button("🔄 Refresh Monitors").clicked() {
                    self.refresh_monitors();
                }

                ui.separator();

                // Adaptive monitor summary based on data
                let monitor_count = self.monitors.len();
                if monitor_count == 0 {
                    ui.colored_label(egui::Color32::GRAY, "No monitors detected");
                } else if monitor_count == 1 {
                    ui.label("Single monitor");
                } else {
                    ui.label(format!("{} monitors", monitor_count));

                    // Show refresh rate diversity
                    let unique_rates: std::collections::HashSet<u32> = self
                        .monitors
                        .iter()
                        .map(|m| m.current_refresh_rate)
                        .collect();

                    if unique_rates.len() > 1 {
                        ui.separator();
                        ui.colored_label(egui::Color32::YELLOW, "Mixed refresh rates");
                    }
                }
            });

            ui.separator();

            if self.monitors.is_empty() {
                ui.label("No monitors found. Click 'Refresh Monitors' to try again.");
                return;
            }

            // Monitor list - data-adaptive layout
            let mut actions_to_perform: Vec<usize> = Vec::new();
            let mut rate_changes: Vec<(usize, u32)> = Vec::new();

            egui::ScrollArea::vertical().show(ui, |ui| {
                let available_width = ui.available_width();
                let available_height = ui.available_height();

                // Adaptive layout decisions based on data and space
                let monitor_count = self.monitors.len();
                let use_compact_layout = available_width < 500.0;
                let use_grid_layout =
                    monitor_count > 1 && available_width > 800.0 && available_height > 400.0;
                let use_minimal_layout = monitor_count > 3 && available_height < 600.0;

                if use_grid_layout {
                    // Grid layout for multiple monitors on wide screens
                    let columns = if monitor_count <= 2 { 2 } else { 3 };
                    egui::Grid::new("monitor_grid")
                        .num_columns(columns)
                        .spacing([10.0, 10.0])
                        .show(ui, |ui| {
                            for (index, monitor) in self.monitors.iter().enumerate() {
                                self.render_monitor_card_grid(
                                    ui,
                                    index,
                                    monitor,
                                    &mut actions_to_perform,
                                    &mut rate_changes,
                                );

                                if (index + 1) % columns == 0 {
                                    ui.end_row();
                                }
                            }
                        });
                } else {
                    // Vertical list layout
                    for (index, monitor) in self.monitors.iter().enumerate() {
                        if use_minimal_layout {
                            self.render_monitor_card_minimal(
                                ui,
                                index,
                                monitor,
                                &mut actions_to_perform,
                                &mut rate_changes,
                            );
                        } else if use_compact_layout {
                            self.render_monitor_card_compact(
                                ui,
                                index,
                                monitor,
                                &mut actions_to_perform,
                                &mut rate_changes,
                            );
                        } else {
                            self.render_monitor_card_standard(
                                ui,
                                index,
                                monitor,
                                &mut actions_to_perform,
                                &mut rate_changes,
                            );
                        }
                        ui.add_space(8.0);
                    }
                }
            });

            // Process collected actions
            for (index, rate) in rate_changes {
                self.selected_rates.insert(index, rate);
            }

            for index in actions_to_perform {
                self.apply_rate_change(index);
            }

            ui.separator();
            ui.small("Auto-refreshes every 5 seconds • HertzRate v0.1.0");
        });
    }
}

pub fn run_gui() -> Result<()> {
    // Get initial monitor count to size window appropriately
    let initial_monitor_count = Monitor::enumerate_monitors().map(|m| m.len()).unwrap_or(1);

    // Adaptive initial window size based on expected content
    let (width, height) = match initial_monitor_count {
        0..=1 => (550.0, 400.0), // Single monitor - compact
        2 => (650.0, 500.0),     // Dual monitor - standard
        3..=4 => (750.0, 600.0), // Multiple monitors - larger
        _ => (850.0, 700.0),     // Many monitors - maximum
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([width, height])
            .with_min_inner_size([400.0, 300.0])
            .with_max_inner_size([1200.0, 800.0])
            .with_resizable(true)
            .with_icon(eframe::icon_data::from_png_bytes(&[]).unwrap_or_default()),
        ..Default::default()
    };

    eframe::run_native(
        "HertzRate - Monitor Refresh Rate Manager",
        options,
        Box::new(|_cc| Ok(Box::new(HertzRateApp::new()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run GUI: {}", e))
}
