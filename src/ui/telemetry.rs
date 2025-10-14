use eframe::egui;
use std::collections::HashSet;
use crate::data::{XPlaneData, HoldingPattern, VorInfo, DisplayMode};
use crate::calculations::calculate_distance;

pub struct SimulatedTelemetryActions {
    pub generate_position: bool,
    pub calculate_result: bool,
    pub change_vor: Option<usize>,
}

pub fn draw_simulated_telemetry(
    ui: &mut egui::Ui,
    simulated_data: &mut XPlaneData,
    simulated_holding: &HoldingPattern,
    available_vors: &[VorInfo],
    selected_vor_index: usize,
    country_filter: &mut String,
    zoom: &mut u8,
    display_mode: &mut DisplayMode,
) -> SimulatedTelemetryActions {
    let mut actions = SimulatedTelemetryActions {
        generate_position: false,
        calculate_result: false,
        change_vor: None,
    };

    ui.heading(egui::RichText::new("VOR Selection").size(16.0));
    ui.add_space(8.0);

    let countries: Vec<String> = {
        let mut countries_set = HashSet::new();
        countries_set.insert("All".to_string());
        for vor in available_vors {
            countries_set.insert(vor.country.clone());
        }
        let mut countries: Vec<String> = countries_set.into_iter().collect();
        countries.sort();
        countries
    };

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Country:").size(13.0));
        egui::ComboBox::new("country_filter", "")
            .selected_text(&*country_filter)
            .width(200.0)
            .show_ui(ui, |ui| {
                for country in &countries {
                    ui.selectable_value(country_filter, country.clone(), country);
                }
            });
    });

    ui.add_space(12.0);

    let filtered_vors: Vec<(usize, &VorInfo)> = available_vors
        .iter()
        .enumerate()
        .filter(|(_, vor)| *country_filter == "All" || vor.country == *country_filter)
        .collect();

    let current_vor = &available_vors[selected_vor_index];
    let current_vor_display = format!("{} - {}", current_vor.id, current_vor.name);

    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("VOR:").size(13.0));
        egui::ComboBox::new("vor_selector", "")
            .selected_text(&current_vor_display)
            .width(200.0)
            .show_ui(ui, |ui| {
                ui.set_min_width(250.0);
                for (idx, vor) in &filtered_vors {
                    let vor_display = format!("{} - {}", vor.id, vor.name);
                    if ui.selectable_label(selected_vor_index == *idx, &vor_display).clicked() {
                        actions.change_vor = Some(*idx);
                    }
                }
            });
    });

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(12.0);

    if !simulated_holding.active {
        if ui.add_sized([280.0, 50.0], egui::Button::new(egui::RichText::new("New Holding").size(16.0))).clicked() {
            actions.generate_position = true;
        }
        ui.add_space(12.0);
        ui.label(egui::RichText::new("Click to generate a random scenario").size(14.0));
        return actions;
    } else {
        ui.horizontal(|ui| {
            if ui.add_sized([135.0, 45.0], egui::Button::new(egui::RichText::new("New").size(15.0))).clicked() {
                actions.generate_position = true;
            }
            if ui.add_sized([135.0, 45.0], egui::Button::new(egui::RichText::new("Result").size(15.0))).clicked() {
                actions.calculate_result = true;
            }
        });
    }

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(12.0);

    ui.label(egui::RichText::new("Aircraft Heading:").size(15.0));
    ui.add_space(6.0);
    let mut heading = simulated_data.aircraft_heading as f32;
    if ui.add(egui::Slider::new(&mut heading, 0.0..=359.0).suffix("°")).changed() {
        simulated_data.aircraft_heading = heading as f64;
    }

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(12.0);

    ui.label(egui::RichText::new("Display Mode:").size(15.0));
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        let button_text = match *display_mode {
            DisplayMode::Radial => "Radial",
            DisplayMode::Cardinal => "Cardinal",
        };
        if ui.button(format!("{} (Toggle)", button_text)).clicked() {
            *display_mode = match *display_mode {
                DisplayMode::Radial => DisplayMode::Cardinal,
                DisplayMode::Cardinal => DisplayMode::Radial,
            };
        }
    });

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(12.0);

    ui.label(egui::RichText::new("Map Zoom:").size(15.0));
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        if ui.add_sized([60.0, 35.0], egui::Button::new(egui::RichText::new("-").size(18.0))).clicked() && *zoom > 8 {
            *zoom -= 1;
        }
        ui.label(egui::RichText::new(format!("Level {}", zoom)).size(14.0));
        if ui.add_sized([60.0, 35.0], egui::Button::new(egui::RichText::new("+").size(18.0))).clicked() && *zoom < 16 {
            *zoom += 1;
        }
    });

    ui.add_space(8.0);
    ui.label(egui::RichText::new("Drag map to move it").size(12.0).color(egui::Color32::from_rgb(150, 150, 150)));

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(12.0);

    ui.heading(egui::RichText::new("Telemetry").size(18.0));
    ui.add_space(10.0);

    ui.group(|ui| {
        ui.label(egui::RichText::new(format!("VOR: {}", simulated_data.vor_id)).size(15.0).color(egui::Color32::from_rgb(0, 200, 255)));
        ui.add_space(3.0);
        ui.label(egui::RichText::new(format!("Freq: {:.2} MHz", simulated_data.vor_freq as f64 / 100.0)).size(14.0).color(egui::Color32::from_rgb(100, 220, 255)));

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);

        let distance = calculate_distance(
            simulated_data.aircraft_lat,
            simulated_data.aircraft_lon,
            simulated_data.vor_lat,
            simulated_data.vor_lon,
        );
        ui.label(egui::RichText::new(format!("Distance: {:.2} NM", distance)).size(15.0).color(egui::Color32::from_rgb(100, 255, 255)));

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);

        ui.label(egui::RichText::new(format!("Heading: {:03.0}°", simulated_data.aircraft_heading)).size(14.0).color(egui::Color32::from_rgb(255, 165, 0)));
        ui.add_space(3.0);
        ui.label(egui::RichText::new(format!("Speed: {:.0} kts", simulated_data.aircraft_groundspeed)).size(14.0).color(egui::Color32::from_rgb(255, 165, 0)));

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);

        ui.label(egui::RichText::new(format!("Lat: {:.6}°  Lon: {:.6}°", simulated_data.aircraft_lat, simulated_data.aircraft_lon)).size(13.0));
    });

    ui.add_space(20.0);
    ui.separator();
    ui.add_space(8.0);

    ui.vertical_centered(|ui| {
        let kofi_button = egui::Button::new(
            egui::RichText::new("Ko-fi")
                .size(11.0)
                .color(egui::Color32::WHITE)
        ).fill(egui::Color32::from_rgb(255, 95, 95));

        if ui.add_sized([70.0, 26.0], kofi_button).clicked() {
            let _ = open::that("https://ko-fi.com/jgananb");
        }
    });

    actions
}

pub struct TelemetryActions {
    pub generate_holding: bool,
    pub calculate_result: bool,
}

pub fn draw_telemetry(
    ui: &mut egui::Ui,
    xplane: &XPlaneData,
    holding: &HoldingPattern,
    zoom: &mut u8,
    display_mode: &mut DisplayMode,
) -> TelemetryActions {
    let mut actions = TelemetryActions {
        generate_holding: false,
        calculate_result: false,
    };

    if !holding.active {
        if ui.add_sized([280.0, 50.0], egui::Button::new(egui::RichText::new("New Holding").size(16.0))).clicked() {
            actions.generate_holding = true;
        }
        ui.add_space(12.0);
        ui.label(egui::RichText::new("Tune a VOR in NAV1").size(14.0));
        return actions;
    } else {
        ui.horizontal(|ui| {
            if ui.add_sized([135.0, 45.0], egui::Button::new(egui::RichText::new("New").size(15.0))).clicked() {
                actions.generate_holding = true;
            }
            if holding.entry_captured {
                if ui.add_sized([135.0, 45.0], egui::Button::new(egui::RichText::new("Result").size(15.0))).clicked() {
                    actions.calculate_result = true;
                }
            } else {
                ui.add_enabled_ui(false, |ui| {
                    ui.add_sized([135.0, 45.0], egui::Button::new(egui::RichText::new("Result").size(15.0)));
                });
            }
        });
    }

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(12.0);

    ui.label(egui::RichText::new("Display Mode:").size(15.0));
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        let button_text = match *display_mode {
            DisplayMode::Radial => "Radial",
            DisplayMode::Cardinal => "Cardinal",
        };
        if ui.button(format!("{} (Toggle)", button_text)).clicked() {
            *display_mode = match *display_mode {
                DisplayMode::Radial => DisplayMode::Cardinal,
                DisplayMode::Cardinal => DisplayMode::Radial,
            };
        }
    });

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(12.0);

    ui.label(egui::RichText::new("Map Zoom:").size(15.0));
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        if ui.add_sized([60.0, 35.0], egui::Button::new(egui::RichText::new("-").size(18.0))).clicked() && *zoom > 8 {
            *zoom -= 1;
        }
        ui.label(egui::RichText::new(format!("Level {}", zoom)).size(14.0));
        if ui.add_sized([60.0, 35.0], egui::Button::new(egui::RichText::new("+").size(18.0))).clicked() && *zoom < 16 {
            *zoom += 1;
        }
    });

    ui.add_space(8.0);
    ui.label(egui::RichText::new("Drag map to move it").size(12.0).color(egui::Color32::from_rgb(150, 150, 150)));

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(12.0);

    ui.heading(egui::RichText::new("Telemetry").size(18.0));
    ui.add_space(10.0);

    ui.group(|ui| {
        ui.label(egui::RichText::new(format!("VOR: {}", xplane.vor_id)).size(15.0).color(egui::Color32::from_rgb(0, 200, 255)));
        ui.add_space(3.0);
        ui.label(egui::RichText::new(format!("Freq: {:.2} MHz", xplane.vor_freq as f64 / 100.0)).size(14.0).color(egui::Color32::from_rgb(100, 220, 255)));

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);

        let distance = calculate_distance(
            xplane.aircraft_lat,
            xplane.aircraft_lon,
            xplane.vor_lat,
            xplane.vor_lon,
        );
        ui.label(egui::RichText::new(format!("Distance: {:.2} NM", distance)).size(15.0).color(egui::Color32::from_rgb(100, 255, 255)));

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);

        ui.label(egui::RichText::new(format!("Heading: {:03.0}°", xplane.aircraft_heading)).size(14.0).color(egui::Color32::from_rgb(255, 165, 0)));
        ui.add_space(3.0);
        ui.label(egui::RichText::new(format!("Speed: {:.0} kts", xplane.aircraft_groundspeed)).size(14.0).color(egui::Color32::from_rgb(255, 165, 0)));

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);

        ui.label(egui::RichText::new(format!("Lat: {:.6}°  Lon: {:.6}°", xplane.aircraft_lat, xplane.aircraft_lon)).size(13.0));
    });

    ui.add_space(16.0);

    if holding.entry_captured {
        ui.add_space(16.0);
        ui.heading(egui::RichText::new("Tracking").size(18.0));
        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label(egui::RichText::new(format!("Points: {}", holding.track_points.len())).size(14.0));
            ui.add_space(3.0);
            if let (Some(first), Some(last)) = (holding.track_points.first(), holding.track_points.last()) {
                ui.label(egui::RichText::new(format!("Time: {:.1}s", last.time - first.time)).size(14.0));
            }

            if !holding.track_points.is_empty() {
                if let Some(last) = holding.track_points.last() {
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new(format!("Last point: {:.6}, {:.6}", last.lat, last.lon)).size(13.0));
                }
            }
        });
    }

    ui.add_space(20.0);
    ui.separator();
    ui.add_space(8.0);

    ui.vertical_centered(|ui| {
        let kofi_button = egui::Button::new(
            egui::RichText::new("Ko-fi")
                .size(11.0)
                .color(egui::Color32::WHITE)
        ).fill(egui::Color32::from_rgb(255, 95, 95));

        if ui.add_sized([70.0, 26.0], kofi_button).clicked() {
            let _ = open::that("https://ko-fi.com/jgananb");
        }
    });

    actions
}
