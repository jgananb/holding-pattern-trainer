use eframe::egui;
use crate::data::{XPlaneData, HoldingPattern, DisplayMode, radial_to_cardinal, bearing_from_radial, bearing_to_from_radial};
use crate::calculations::calculate_distance;
use crate::tile_manager::{TileManager, TileCoord};

pub fn draw_map(
    ui: &mut egui::Ui,
    xplane: &XPlaneData,
    holding: &HoldingPattern,
    tile_manager: &TileManager,
    zoom: u8,
    show_overlay: bool,
    map_offset: &mut egui::Vec2,
    is_dragging: &mut bool,
    drag_start: &mut Option<egui::Pos2>,
    display_mode: DisplayMode,
) {
    let available = ui.available_size();
    let (response, painter) = ui.allocate_painter(available, egui::Sense::click_and_drag());
    let rect = response.rect;

    if response.drag_started() {
        *is_dragging = true;
        *drag_start = Some(response.interact_pointer_pos().unwrap_or(rect.center()));
    }

    if response.dragged() {
        if let Some(start_pos) = *drag_start {
            if let Some(current_pos) = response.interact_pointer_pos() {
                let delta = current_pos - start_pos;
                *map_offset += delta;
                *drag_start = Some(current_pos);
            }
        }
    }

    if response.drag_stopped() {
        *is_dragging = false;
        *drag_start = None;
    }

    painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(20, 25, 30));

    if !holding.active || xplane.vor_lat == 0.0 {
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "Press 'New Holding' to start",
            egui::FontId::proportional(24.0),
            egui::Color32::GRAY,
        );
        return;
    }

    let center_lat = xplane.vor_lat;
    let center_lon = xplane.vor_lon;

    let (center_tile_x, center_tile_y) = TileManager::lat_lon_to_tile(center_lat, center_lon, zoom);

    let tiles_x = 4;
    let tiles_y = 3;
    let tile_size = 256.0;

    for dy in -(tiles_y as i32)..(tiles_y as i32) {
        for dx in -(tiles_x as i32)..(tiles_x as i32) {
            let tile_x = (center_tile_x as i32 + dx) as u32;
            let tile_y = (center_tile_y as i32 + dy) as u32;

            let coord = TileCoord {
                zoom,
                x: tile_x,
                y: tile_y,
            };

            if let Some(texture) = tile_manager.get_or_load_tile(coord, ui.ctx()) {
                let (tile_lat, tile_lon) = TileManager::tile_to_lat_lon(tile_x, tile_y, zoom);
                let tile_pos = lat_lon_to_screen(tile_lat, tile_lon, center_lat, center_lon, zoom, &rect, map_offset);

                let tile_rect = egui::Rect::from_min_size(
                    tile_pos,
                    egui::vec2(tile_size, tile_size)
                );

                painter.image(
                    texture.id(),
                    tile_rect,
                    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    egui::Color32::WHITE,
                );
            }
        }
    }

    let vor_pos = lat_lon_to_screen(xplane.vor_lat, xplane.vor_lon, center_lat, center_lon, zoom, &rect, map_offset);
    let cross_size = 20.0;
    painter.line_segment(
        [vor_pos - egui::vec2(cross_size, 0.0), vor_pos + egui::vec2(cross_size, 0.0)],
        egui::Stroke::new(4.0, egui::Color32::from_rgb(0, 200, 255)),
    );
    painter.line_segment(
        [vor_pos - egui::vec2(0.0, cross_size), vor_pos + egui::vec2(0.0, cross_size)],
        egui::Stroke::new(4.0, egui::Color32::from_rgb(0, 200, 255)),
    );
    painter.circle_stroke(vor_pos, 15.0, egui::Stroke::new(4.0, egui::Color32::from_rgb(0, 200, 255)));

    let vor_label = &xplane.vor_id;
    let font_id = egui::FontId::proportional(18.0);
    let galley = painter.layout_no_wrap(vor_label.clone(), font_id.clone(), egui::Color32::WHITE);
    let text_pos = vor_pos + egui::vec2(0.0, -35.0);
    let text_rect = egui::Align2::CENTER_CENTER.anchor_rect(egui::Rect::from_min_size(text_pos, galley.size()));
    painter.rect_filled(text_rect.expand(4.0), 3.0, egui::Color32::from_rgba_premultiplied(0, 0, 0, 180));
    painter.galley(text_rect.min, galley, egui::Color32::WHITE);

    if holding.active && !holding.correct_entry.is_empty() {
        let radial_angle = (holding.radial as f64).to_radians();
        let radial_length = 250.0;
        let radial_end = vor_pos + egui::vec2(
            (radial_angle.sin() * radial_length) as f32,
            -(radial_angle.cos() * radial_length) as f32,
        );
        painter.line_segment(
            [vor_pos, radial_end],
            egui::Stroke::new(2.5, egui::Color32::from_rgb(0, 180, 255)),
        );

        let sector_radius = 180.0;

        if holding.right_turns {
            let direct_start = (holding.outbound_course - 90.0 + 360.0) % 360.0;
            draw_sector_filled(&painter, vor_pos, direct_start.to_radians(), std::f64::consts::PI,
                               sector_radius, egui::Color32::from_rgba_premultiplied(0, 255, 0, 50), "DIRECT");

            let teardrop_start = (holding.outbound_course + 90.0) % 360.0;
            draw_sector_filled(&painter, vor_pos, teardrop_start.to_radians(), 70.0_f64.to_radians(),
                               sector_radius, egui::Color32::from_rgba_premultiplied(255, 0, 255, 50), "TEARDROP");

            let parallel_start = (holding.outbound_course + 160.0) % 360.0;
            draw_sector_filled(&painter, vor_pos, parallel_start.to_radians(), 110.0_f64.to_radians(),
                               sector_radius, egui::Color32::from_rgba_premultiplied(100, 150, 255, 50), "PARALLEL");

        } else {
            let direct_start = (holding.outbound_course - 90.0 + 360.0) % 360.0;
            draw_sector_filled(&painter, vor_pos, direct_start.to_radians(), std::f64::consts::PI,
                               sector_radius, egui::Color32::from_rgba_premultiplied(0, 255, 0, 50), "DIRECT");

            let parallel_start = (holding.outbound_course + 90.0) % 360.0;
            draw_sector_filled(&painter, vor_pos, parallel_start.to_radians(), 110.0_f64.to_radians(),
                               sector_radius, egui::Color32::from_rgba_premultiplied(100, 150, 255, 50), "PARALLEL");

            let teardrop_start = (holding.outbound_course + 200.0) % 360.0;
            draw_sector_filled(&painter, vor_pos, teardrop_start.to_radians(), 70.0_f64.to_radians(),
                               sector_radius, egui::Color32::from_rgba_premultiplied(255, 0, 255, 50), "TEARDROP");
        }
    }

    if holding.entry_captured && holding.entry_lat != 0.0 && holding.entry_lon != 0.0 {
        let entry_pos = lat_lon_to_screen(
            holding.entry_lat,
            holding.entry_lon,
            center_lat,
            center_lon,
            zoom,
            &rect,
            map_offset
        );

        painter.circle_filled(entry_pos, 8.0, egui::Color32::from_rgb(255, 255, 0));
        painter.circle_stroke(entry_pos, 8.0, egui::Stroke::new(3.0, egui::Color32::from_rgb(200, 200, 0)));
        painter.circle_stroke(entry_pos, 12.0, egui::Stroke::new(2.0, egui::Color32::from_rgba_premultiplied(255, 255, 0, 150)));

        if !holding.correct_entry.is_empty() {
            let entry_label = &holding.correct_entry;
            let font_id = egui::FontId::monospace(12.0);
            let galley = painter.layout_no_wrap(entry_label.clone(), font_id.clone(), egui::Color32::WHITE);
            let text_pos = entry_pos + egui::vec2(0.0, -25.0);
            let text_rect = egui::Align2::CENTER_CENTER.anchor_rect(egui::Rect::from_min_size(text_pos, galley.size()));
            painter.rect_filled(text_rect.expand(4.0), 3.0, egui::Color32::from_rgba_premultiplied(0, 0, 0, 200));
            painter.galley(text_rect.min, galley, egui::Color32::from_rgb(255, 255, 0));
        }

        let heading_rad = holding.start_heading.to_radians();
        let heading_line_end = entry_pos + egui::vec2(
            (heading_rad.sin() * 40.0) as f32,
            -(heading_rad.cos() * 40.0) as f32,
        );
        painter.line_segment(
            [entry_pos, heading_line_end],
            egui::Stroke::new(3.0, egui::Color32::from_rgb(255, 255, 0))
        );
        painter.circle_filled(heading_line_end, 4.0, egui::Color32::from_rgb(255, 255, 0));
    }

    if holding.track_points.len() > 1 {
        let points: Vec<egui::Pos2> = holding.track_points
            .iter()
            .map(|p| lat_lon_to_screen(p.lat, p.lon, center_lat, center_lon, zoom, &rect, map_offset))
            .collect();

        painter.add(egui::Shape::line(
            points.clone(),
            egui::Stroke::new(5.0, egui::Color32::from_rgba_premultiplied(0, 255, 255, 220)),
        ));

        let arrow_color = egui::Color32::from_rgba_premultiplied(0, 255, 255, 200);
        let arrow_interval = 8;

        for i in (arrow_interval..points.len()).step_by(arrow_interval) {
            let p1 = points[i - 1];
            let p2 = points[i];

            let dx = p2.x - p1.x;
            let dy = p2.y - p1.y;
            let angle = dy.atan2(dx);

            let mid_point = egui::pos2((p1.x + p2.x) / 2.0, (p1.y + p2.y) / 2.0);

            draw_arrow(&painter, mid_point, angle, arrow_color);
        }
    }

    let aircraft_pos = lat_lon_to_screen(xplane.aircraft_lat, xplane.aircraft_lon, center_lat, center_lon, zoom, &rect, map_offset);
    draw_aircraft_icon(&painter, aircraft_pos, xplane.aircraft_heading);

    if show_overlay && holding.active {
        draw_atc_overlay(ui, &painter, &rect, xplane, holding, display_mode);
    }
}

pub fn draw_atc_overlay(
    _ui: &mut egui::Ui,
    painter: &egui::Painter,
    rect: &egui::Rect,
    xplane: &XPlaneData,
    holding: &HoldingPattern,
    display_mode: DisplayMode,
) {
    let overlay_pos = rect.min + egui::vec2(15.0, 15.0);
    let overlay_width = 400.0;

    let turns_text = if holding.right_turns { "RIGHT TURNS" } else { "LEFT TURNS" };

    let mut y_offset = 0.0;
    let line_height = 17.0;

    let bg_height = if holding.entry_captured {
        if !holding.correct_entry.is_empty() { 165.0 } else { 140.0 }
    } else {
        130.0
    };

    let bg_rect = egui::Rect::from_min_size(overlay_pos, egui::vec2(overlay_width, bg_height));
    painter.rect_filled(bg_rect.expand(8.0), 4.0, egui::Color32::from_rgba_premultiplied(0, 20, 40, 220));
    painter.rect_stroke(bg_rect.expand(8.0), 4.0, egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 180, 255)));

    let text_start = overlay_pos + egui::vec2(10.0, 10.0);

    let title = "HOLDING INSTRUCTION";
    let title_galley = painter.layout_no_wrap(title.to_string(), egui::FontId::monospace(13.0), egui::Color32::from_rgb(100, 200, 255));
    painter.galley(text_start + egui::vec2(0.0, y_offset), title_galley, egui::Color32::from_rgb(100, 200, 255));
    y_offset += line_height * 1.4;

    let holding_text = match display_mode {
        DisplayMode::Radial => {
            format!("HOLD AT {} R-{:03} {}", xplane.vor_id, holding.radial, turns_text)
        }
        DisplayMode::Cardinal => {
            let cardinal = radial_to_cardinal(holding.radial);
            format!("HOLD {} OF {} {}", cardinal, xplane.vor_id, turns_text)
        }
    };
    let holding_galley = painter.layout_no_wrap(holding_text, egui::FontId::monospace(12.0), egui::Color32::from_rgb(200, 255, 200));
    painter.galley(text_start + egui::vec2(0.0, y_offset), holding_galley, egui::Color32::from_rgb(200, 255, 200));
    y_offset += line_height * 1.3;

    painter.line_segment(
        [text_start + egui::vec2(0.0, y_offset), text_start + egui::vec2(overlay_width - 20.0, y_offset)],
        egui::Stroke::new(1.0, egui::Color32::from_rgb(80, 120, 150))
    );
    y_offset += line_height * 0.7;

    let from_bearing = bearing_from_radial(holding.radial);
    let to_bearing = bearing_to_from_radial(holding.radial);
    let courses_text = format!("IN: {:03.0}°  OUT: {:03.0}° | {:03}° FROM {} | {:03}° TO {}",
        holding.inbound_course, holding.outbound_course, from_bearing, xplane.vor_id, to_bearing, xplane.vor_id);
    let courses_galley = painter.layout_no_wrap(courses_text, egui::FontId::monospace(11.0), egui::Color32::from_rgb(180, 200, 220));
    painter.galley(text_start + egui::vec2(0.0, y_offset), courses_galley, egui::Color32::from_rgb(180, 200, 220));
    y_offset += line_height * 1.1;

    if holding.entry_captured {
        painter.line_segment(
            [text_start + egui::vec2(0.0, y_offset), text_start + egui::vec2(overlay_width - 20.0, y_offset)],
            egui::Stroke::new(1.0, egui::Color32::from_rgb(80, 120, 150))
        );
        y_offset += line_height * 0.7;

        let entry_h = format!("Entry HDG: {:03.0}°", holding.start_heading);
        let entry_h_galley = painter.layout_no_wrap(entry_h, egui::FontId::monospace(11.0), egui::Color32::from_rgb(200, 220, 255));
        painter.galley(text_start + egui::vec2(0.0, y_offset), entry_h_galley, egui::Color32::from_rgb(200, 220, 255));
        y_offset += line_height * 1.2;

        if !holding.correct_entry.is_empty() {
            let entry_text = format!(" ENTRY: {} ", holding.correct_entry);
            let entry_galley = painter.layout_no_wrap(entry_text.clone(), egui::FontId::monospace(12.0), egui::Color32::BLACK);
            let entry_size = entry_galley.size();

            let entry_box_pos = text_start + egui::vec2((overlay_width - entry_size.x - 28.0) / 2.0, y_offset);
            let entry_rect = egui::Rect::from_min_size(entry_box_pos, egui::vec2(entry_size.x + 16.0, entry_size.y + 10.0));

            painter.rect_filled(entry_rect, 3.0, egui::Color32::from_rgb(100, 255, 100));
            painter.rect_stroke(entry_rect, 3.0, egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 200, 0)));
            painter.galley(entry_box_pos + egui::vec2(8.0, 5.0), entry_galley, egui::Color32::BLACK);
        }
    } else {
        let distance = calculate_distance(
            xplane.aircraft_lat,
            xplane.aircraft_lon,
            xplane.vor_lat,
            xplane.vor_lon,
        );

        painter.line_segment(
            [text_start + egui::vec2(0.0, y_offset), text_start + egui::vec2(overlay_width - 20.0, y_offset)],
            egui::Stroke::new(1.0, egui::Color32::from_rgb(80, 120, 150))
        );
        y_offset += line_height * 0.7;

        let dist_text = format!("Distance: {:.1} NM", distance);
        let dist_galley = painter.layout_no_wrap(dist_text, egui::FontId::monospace(11.0), egui::Color32::from_rgb(255, 255, 150));
        painter.galley(text_start + egui::vec2(0.0, y_offset), dist_galley, egui::Color32::from_rgb(255, 255, 150));
        y_offset += line_height * 0.95;

        let capture_text = "(Capture at 5 NM)";
        let capture_galley = painter.layout_no_wrap(capture_text.to_string(), egui::FontId::monospace(10.0), egui::Color32::from_rgb(180, 180, 180));
        painter.galley(text_start + egui::vec2(0.0, y_offset), capture_galley, egui::Color32::from_rgb(180, 180, 180));
    }
}

pub fn draw_aircraft_icon(painter: &egui::Painter, pos: egui::Pos2, heading: f64) {
    let heading_rad = heading.to_radians();

    let nose_len = 14.0;
    let wing_len = 10.0;
    let tail_len = 6.0;

    let nose = pos + egui::vec2(
        (heading_rad.sin() * nose_len) as f32,
        -(heading_rad.cos() * nose_len) as f32,
    );

    let wing_angle = std::f64::consts::PI * 0.7;
    let wing_left = pos + egui::vec2(
        ((heading_rad + wing_angle).sin() * wing_len) as f32,
        -((heading_rad + wing_angle).cos() * wing_len) as f32,
    );
    let wing_right = pos + egui::vec2(
        ((heading_rad - wing_angle).sin() * wing_len) as f32,
        -((heading_rad - wing_angle).cos() * wing_len) as f32,
    );

    let tail = pos + egui::vec2(
        ((heading_rad + std::f64::consts::PI).sin() * tail_len) as f32,
        -((heading_rad + std::f64::consts::PI).cos() * tail_len) as f32,
    );

    let aircraft_shape = vec![nose, wing_left, tail, wing_right];

    painter.add(egui::Shape::convex_polygon(
        aircraft_shape.clone(),
        egui::Color32::from_rgb(50, 150, 255),
        egui::Stroke::new(3.0, egui::Color32::from_rgb(0, 50, 100)),
    ));
}

pub fn draw_sector_filled(painter: &egui::Painter, center: egui::Pos2, start_angle: f64, arc_angle: f64, radius: f32, color: egui::Color32, label: &str) {
    let segments = 40;
    let mut points = vec![center];

    for i in 0..=segments {
        let t = i as f64 / segments as f64;
        let angle = start_angle + arc_angle * t;
        let x = center.x + (angle.sin() * radius as f64) as f32;
        let y = center.y - (angle.cos() * radius as f64) as f32;
        points.push(egui::pos2(x, y));
    }
    points.push(center);

    painter.add(egui::Shape::convex_polygon(
        points,
        color,
        egui::Stroke::new(1.5, egui::Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 120)),
    ));

    let mid_angle = start_angle + arc_angle / 2.0;
    let label_distance = radius * 0.6;
    let label_pos = egui::pos2(
        center.x + (mid_angle.sin() * label_distance as f64) as f32,
        center.y - (mid_angle.cos() * label_distance as f64) as f32,
    );

    let galley = painter.layout_no_wrap(
        label.to_string(),
        egui::FontId::monospace(13.0),
        egui::Color32::WHITE,
    );

    let text_rect = egui::Align2::CENTER_CENTER.anchor_rect(egui::Rect::from_min_size(label_pos, galley.size()));
    painter.rect_filled(text_rect.expand(3.0), 2.0, egui::Color32::from_rgba_premultiplied(0, 0, 0, 200));
    painter.galley(text_rect.min, galley, egui::Color32::WHITE);
}

pub fn draw_arrow(painter: &egui::Painter, pos: egui::Pos2, angle: f32, color: egui::Color32) {
    let arrow_size = 8.0;
    let arrow_angle = 0.5;

    let tip = pos;

    let left = egui::pos2(
        tip.x - (angle.cos() * arrow_size - (angle + arrow_angle).sin() * arrow_size),
        tip.y - (angle.sin() * arrow_size + (angle + arrow_angle).cos() * arrow_size),
    );

    let right = egui::pos2(
        tip.x - (angle.cos() * arrow_size - (angle - arrow_angle).sin() * arrow_size),
        tip.y - (angle.sin() * arrow_size + (angle - arrow_angle).cos() * arrow_size),
    );

    painter.add(egui::Shape::convex_polygon(
        vec![tip, left, right],
        color,
        egui::Stroke::new(1.5, color),
    ));
}

fn lat_lon_to_screen(lat: f64, lon: f64, center_lat: f64, center_lon: f64, zoom: u8, rect: &egui::Rect, map_offset: &egui::Vec2) -> egui::Pos2 {
    let scale = 2_f64.powi(zoom as i32) * 256.0;

    let center_x = ((center_lon + 180.0) / 360.0 * scale) as f32;
    let center_y = ((1.0 - (center_lat.to_radians().tan() + 1.0 / center_lat.to_radians().cos()).ln() / std::f64::consts::PI) / 2.0 * scale) as f32;

    let point_x = ((lon + 180.0) / 360.0 * scale) as f32;
    let point_y = ((1.0 - (lat.to_radians().tan() + 1.0 / lat.to_radians().cos()).ln() / std::f64::consts::PI) / 2.0 * scale) as f32;

    let dx = point_x - center_x;
    let dy = point_y - center_y;

    egui::pos2(rect.center().x + dx + map_offset.x, rect.center().y + dy + map_offset.y)
}
