#![windows_subsystem = "windows"]

use eframe::egui;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

mod data;
mod calculations;
mod tile_manager;
mod ui;

use data::{XPlaneData, HoldingPattern, VorInfo, Tab, DisplayMode};
use calculations::{calculate_distance, calculate_bearing, calculate_entry_type};
use tile_manager::TileManager;

struct HoldingViewerApp {
    xplane_data: Arc<Mutex<XPlaneData>>,
    holding: Arc<Mutex<HoldingPattern>>,
    data_file_path: PathBuf,
    tile_manager: TileManager,
    zoom: u8,
    show_overlay: bool,
    map_offset: egui::Vec2,
    is_dragging: bool,
    drag_start: Option<egui::Pos2>,
    active_tab: Tab,
    simulated_data: XPlaneData,
    simulated_holding: HoldingPattern,
    show_about: bool,
    show_how_it_works: bool,
    available_vors: Vec<VorInfo>,
    selected_vor_index: usize,
    country_filter: String,
    display_mode: DisplayMode,
}

impl HoldingViewerApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let temp_dir = std::env::var("TEMP")
            .or_else(|_| std::env::var("TMP"))
            .unwrap_or_else(|_| {
                let appdata = std::env::var("LOCALAPPDATA")
                    .unwrap_or_else(|_| std::env::var("USERPROFILE").unwrap_or_else(|_| String::from("C:\\")));
                format!("{}\\Temp", appdata)
            });
        let data_file_path = PathBuf::from(temp_dir).join("xplane_data.json");

        let available_vors = Self::load_vors();
        let first_vor = &available_vors[0];

        let mut sim_data = XPlaneData::default();
        sim_data.vor_id = first_vor.id.clone();
        sim_data.vor_freq = first_vor.freq;
        sim_data.vor_lat = first_vor.lat;
        sim_data.vor_lon = first_vor.lon;

        let app = Self {
            xplane_data: Arc::new(Mutex::new(XPlaneData::default())),
            holding: Arc::new(Mutex::new(HoldingPattern::default())),
            data_file_path,
            tile_manager: TileManager::new(),
            zoom: 11,
            show_overlay: true,
            map_offset: egui::Vec2::ZERO,
            is_dragging: false,
            drag_start: None,
            active_tab: Tab::Simulate,
            simulated_data: sim_data,
            simulated_holding: HoldingPattern::default(),
            show_about: false,
            show_how_it_works: false,
            available_vors,
            selected_vor_index: 0,
            country_filter: "All".to_string(),
            display_mode: DisplayMode::Radial,
        };

        let xplane_clone = app.xplane_data.clone();
        let holding_clone = app.holding.clone();
        let path_clone = app.data_file_path.clone();
        let ctx_clone = cc.egui_ctx.clone();

        std::thread::spawn(move || loop {
            if let Ok(content) = fs::read_to_string(&path_clone) {
                if let Ok(parsed_data) = serde_json::from_str::<XPlaneData>(&content) {
                    if let Ok(mut xplane) = xplane_clone.lock() {
                        *xplane = parsed_data.clone();

                        if let Ok(mut holding) = holding_clone.lock() {
                            if holding.active && xplane.vor_lat != 0.0 {
                                let distance = calculate_distance(
                                    xplane.aircraft_lat,
                                    xplane.aircraft_lon,
                                    xplane.vor_lat,
                                    xplane.vor_lon,
                                );

                                if !holding.entry_captured && distance <= 5.0 && distance < holding.last_distance {
                                    let bearing_to_vor = calculate_bearing(
                                        xplane.aircraft_lat,
                                        xplane.aircraft_lon,
                                        xplane.vor_lat,
                                        xplane.vor_lon,
                                    );

                                    let heading_diff = ((xplane.aircraft_heading - bearing_to_vor + 360.0) % 360.0).abs();
                                    let heading_diff = if heading_diff > 180.0 { 360.0 - heading_diff } else { heading_diff };

                                    if heading_diff <= 90.0 {
                                        holding.start_heading = xplane.aircraft_heading;
                                        holding.entry_lat = xplane.aircraft_lat;
                                        holding.entry_lon = xplane.aircraft_lon;
                                        holding.entry_captured = true;
                                    }
                                }

                                holding.last_distance = distance;

                                if holding.entry_captured {
                                    holding.track_points.push(data::TrackPoint {
                                        lat: xplane.aircraft_lat,
                                        lon: xplane.aircraft_lon,
                                        time: std::time::SystemTime::now()
                                            .duration_since(std::time::UNIX_EPOCH)
                                            .unwrap()
                                            .as_secs_f64(),
                                    });
                                }
                            }
                        }

                        ctx_clone.request_repaint();
                    }
                }
            }
            std::thread::sleep(Duration::from_secs(1));
        });

        app
    }

    fn load_vors() -> Vec<VorInfo> {
        let vors_data = include_str!("../vors_data.txt");
        let mut vors = Vec::new();

        for line in vors_data.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 6 {
                if let (Ok(lat), Ok(lon), Ok(freq)) = (
                    parts[3].parse::<f64>(),
                    parts[4].parse::<f64>(),
                    parts[5].parse::<i32>()
                ) {
                    vors.push(VorInfo {
                        country: parts[0].to_string(),
                        id: parts[1].to_string(),
                        name: parts[2].to_string(),
                        lat,
                        lon,
                        freq,
                    });
                }
            }
        }
        vors
    }

    fn generate_new_holding(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        if let (Ok(xplane), Ok(mut holding)) = (self.xplane_data.lock(), self.holding.lock()) {
            if xplane.vor_id.is_empty() || xplane.vor_lat == 0.0 {
                return;
            }

            holding.active = true;
            holding.radial = rng.gen_range(0..36) * 10;
            holding.right_turns = rng.gen_bool(0.5);
            holding.entry_captured = false;
            holding.start_heading = 0.0;
            holding.entry_lat = 0.0;
            holding.entry_lon = 0.0;
            holding.correct_entry = String::new();
            holding.track_points.clear();

            holding.outbound_course = holding.radial as f64;
            holding.inbound_course = (holding.radial as f64 + 180.0) % 360.0;
            holding.last_distance = 999.0;

            self.show_overlay = true;
            self.map_offset = egui::Vec2::ZERO;
        }
    }

    fn generate_simulated_position(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let radial = rng.gen_range(0..36) * 10;
        let distance_nm = 5.0_f64;

        let bearing_rad = (radial as f64).to_radians();

        let angular_distance: f64 = distance_nm / 3440.065;
        let vor_lat_rad = self.simulated_data.vor_lat.to_radians();
        let vor_lon_rad = self.simulated_data.vor_lon.to_radians();

        let new_lat_rad = (vor_lat_rad.sin() * angular_distance.cos() +
                          vor_lat_rad.cos() * angular_distance.sin() * bearing_rad.cos()).asin();

        let new_lon_rad = vor_lon_rad +
                         (bearing_rad.sin() * angular_distance.sin() * vor_lat_rad.cos())
                         .atan2(angular_distance.cos() - vor_lat_rad.sin() * new_lat_rad.sin());

        self.simulated_data.aircraft_lat = new_lat_rad.to_degrees();
        self.simulated_data.aircraft_lon = new_lon_rad.to_degrees();

        let heading_to_vor = calculate_bearing(
            self.simulated_data.aircraft_lat,
            self.simulated_data.aircraft_lon,
            self.simulated_data.vor_lat,
            self.simulated_data.vor_lon,
        );
        self.simulated_data.aircraft_heading = heading_to_vor;
        self.simulated_data.aircraft_alt = 8000.0;
        self.simulated_data.aircraft_groundspeed = 180.0;

        self.simulated_holding.active = true;
        self.simulated_holding.radial = rng.gen_range(0..36) * 10;
        self.simulated_holding.right_turns = rng.gen_bool(0.5);
        self.simulated_holding.entry_captured = false;
        self.simulated_holding.start_heading = 0.0;
        self.simulated_holding.entry_lat = 0.0;
        self.simulated_holding.entry_lon = 0.0;
        self.simulated_holding.correct_entry = String::new();
        self.simulated_holding.track_points.clear();
        self.simulated_holding.outbound_course = self.simulated_holding.radial as f64;
        self.simulated_holding.inbound_course = (self.simulated_holding.radial as f64 + 180.0) % 360.0;
        self.simulated_holding.last_distance = 999.0;

        self.show_overlay = true;
        self.map_offset = egui::Vec2::ZERO;
    }

    fn calculate_result(&self) {
        if let Ok(mut holding) = self.holding.lock() {
            if !holding.entry_captured || holding.start_heading == 0.0 {
                return;
            }

            holding.correct_entry = calculate_entry_type(
                holding.start_heading,
                holding.inbound_course,
                holding.right_turns
            );
        }
    }

    fn calculate_simulated_result(&mut self) {
        if self.simulated_data.aircraft_lat == 0.0 {
            return;
        }

        self.simulated_holding.start_heading = self.simulated_data.aircraft_heading;
        self.simulated_holding.entry_lat = self.simulated_data.aircraft_lat;
        self.simulated_holding.entry_lon = self.simulated_data.aircraft_lon;
        self.simulated_holding.entry_captured = true;

        self.simulated_holding.correct_entry = calculate_entry_type(
            self.simulated_holding.start_heading,
            self.simulated_holding.inbound_course,
            self.simulated_holding.right_turns
        );
    }

    fn change_selected_vor(&mut self, vor_index: usize) {
        if vor_index < self.available_vors.len() {
            let vor = &self.available_vors[vor_index];
            self.simulated_data.vor_id = vor.id.clone();
            self.simulated_data.vor_freq = vor.freq;
            self.simulated_data.vor_lat = vor.lat;
            self.simulated_data.vor_lon = vor.lon;
            self.selected_vor_index = vor_index;
            self.simulated_data.aircraft_lat = 0.0;
            self.simulated_data.aircraft_lon = 0.0;
            self.simulated_holding = HoldingPattern::default();
            self.map_offset = egui::Vec2::ZERO;
        }
    }
}

impl eframe::App for HoldingViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let xplane = self.xplane_data.lock().unwrap().clone();
        let holding = self.holding.lock().unwrap().clone();

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.show_about = true;
                        ui.close_menu();
                    }
                    if ui.button("How to Fly Holdings").clicked() {
                        self.show_how_it_works = true;
                        ui.close_menu();
                    }
                });
            });
        });

        if self.show_about {
            ui::dialogs::draw_about_dialog(ctx, &mut self.show_about);
        }

        if self.show_how_it_works {
            ui::dialogs::draw_how_to_fly_dialog(ctx, &mut self.show_how_it_works);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.active_tab, Tab::Simulate,
                    egui::RichText::new("SIMULATE").size(14.0));
                ui.selectable_value(&mut self.active_tab, Tab::XPlane11,
                    egui::RichText::new("X-PLANE 11 CONNECT").size(14.0));
            });

            ui.add_space(5.0);
            ui.separator();
            ui.add_space(10.0);

            match self.active_tab {
                Tab::Simulate => {
                    let mut actions = ui::telemetry::SimulatedTelemetryActions {
                        generate_position: false,
                        calculate_result: false,
                        change_vor: None,
                    };

                    ui.horizontal_top(|ui| {
                        let available_width = ui.available_width() - 315.0;

                        ui.vertical(|ui| {
                            ui.set_width(available_width);
                            ui::map::draw_map(
                                ui,
                                &self.simulated_data,
                                &self.simulated_holding,
                                &self.tile_manager,
                                self.zoom,
                                self.show_overlay,
                                &mut self.map_offset,
                                &mut self.is_dragging,
                                &mut self.drag_start,
                                self.display_mode,
                            );
                        });

                        ui.add_space(15.0);

                        ui.vertical(|ui| {
                            ui.set_min_width(300.0);
                            ui.set_max_width(300.0);

                            egui::ScrollArea::vertical()
                                .show(ui, |ui| {
                                    actions = ui::telemetry::draw_simulated_telemetry(
                                        ui,
                                        &mut self.simulated_data,
                                        &self.simulated_holding,
                                        &self.available_vors,
                                        self.selected_vor_index,
                                        &mut self.country_filter,
                                        &mut self.zoom,
                                        &mut self.display_mode,
                                    );
                                });
                        });
                    });

                    if actions.generate_position {
                        self.generate_simulated_position();
                    }
                    if actions.calculate_result {
                        self.calculate_simulated_result();
                    }
                    if let Some(idx) = actions.change_vor {
                        self.change_selected_vor(idx);
                    }
                },
                Tab::XPlane11 => {
                    let mut actions = ui::telemetry::TelemetryActions {
                        generate_holding: false,
                        calculate_result: false,
                    };

                    ui.horizontal_top(|ui| {
                        let available_width = ui.available_width() - 315.0;

                        ui.vertical(|ui| {
                            ui.set_width(available_width);
                            ui::map::draw_map(
                                ui,
                                &xplane,
                                &holding,
                                &self.tile_manager,
                                self.zoom,
                                self.show_overlay,
                                &mut self.map_offset,
                                &mut self.is_dragging,
                                &mut self.drag_start,
                                self.display_mode,
                            );
                        });

                        ui.add_space(15.0);

                        ui.vertical(|ui| {
                            ui.set_min_width(300.0);
                            ui.set_max_width(300.0);

                            egui::ScrollArea::vertical()
                                .show(ui, |ui| {
                                    actions = ui::telemetry::draw_telemetry(
                                        ui,
                                        &xplane,
                                        &holding,
                                        &mut self.zoom,
                                        &mut self.display_mode,
                                    );
                                });
                        });
                    });

                    if actions.generate_holding {
                        self.generate_new_holding();
                    }
                    if actions.calculate_result {
                        self.calculate_result();
                    }
                }
            }
        });

        ctx.request_repaint_after(Duration::from_millis(100));
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 820.0])
            .with_title("Holding Trainer"),
        ..Default::default()
    };

    eframe::run_native(
        "Holding Trainer",
        options,
        Box::new(|cc| Ok(Box::new(HoldingViewerApp::new(cc)))),
    )
}
