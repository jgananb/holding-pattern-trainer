use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XPlaneData {
    pub vor_id: String,
    pub vor_freq: i32,
    pub vor_lat: f64,
    pub vor_lon: f64,
    pub aircraft_lat: f64,
    pub aircraft_lon: f64,
    pub aircraft_alt: f64,
    pub aircraft_heading: f64,
    pub aircraft_groundspeed: f64,
}

impl Default for XPlaneData {
    fn default() -> Self {
        Self {
            vor_id: String::new(),
            vor_freq: 0,
            vor_lat: 0.0,
            vor_lon: 0.0,
            aircraft_lat: 0.0,
            aircraft_lon: 0.0,
            aircraft_alt: 0.0,
            aircraft_heading: 0.0,
            aircraft_groundspeed: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrackPoint {
    pub lat: f64,
    pub lon: f64,
    pub time: f64,
}

#[derive(Debug, Clone)]
pub struct HoldingPattern {
    pub active: bool,
    pub radial: i32,
    pub right_turns: bool,
    pub entry_captured: bool,
    pub start_heading: f64,
    pub entry_lat: f64,
    pub entry_lon: f64,
    pub correct_entry: String,
    pub inbound_course: f64,
    pub outbound_course: f64,
    pub track_points: Vec<TrackPoint>,
    pub last_distance: f64,
}

impl Default for HoldingPattern {
    fn default() -> Self {
        Self {
            active: false,
            radial: 0,
            right_turns: true,
            entry_captured: false,
            start_heading: 0.0,
            entry_lat: 0.0,
            entry_lon: 0.0,
            correct_entry: String::new(),
            inbound_course: 0.0,
            outbound_course: 0.0,
            track_points: Vec::new(),
            last_distance: 999.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VorInfo {
    pub country: String,
    pub id: String,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub freq: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Simulate,
    XPlane11,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisplayMode {
    Radial,
    Cardinal,
}

pub fn radial_to_cardinal(radial: i32) -> &'static str {
    match radial {
        337..=360 | 0..=22 => "NORTH",
        23..=67 => "NORTHEAST",
        68..=112 => "EAST",
        113..=157 => "SOUTHEAST",
        158..=202 => "SOUTH",
        203..=247 => "SOUTHWEST",
        248..=292 => "WEST",
        293..=336 => "NORTHWEST",
        _ => "UNKNOWN",
    }
}

pub fn bearing_from_radial(radial: i32) -> i32 {
    radial
}

pub fn bearing_to_from_radial(radial: i32) -> i32 {
    (radial + 180) % 360
}
