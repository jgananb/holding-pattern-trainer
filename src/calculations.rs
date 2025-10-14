pub fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 3440.065;
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();

    let a = (dlat / 2.0).sin().powi(2) + lat1_rad.cos() * lat2_rad.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    r * c
}

pub fn calculate_bearing(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let dlon = (lon2 - lon1).to_radians();

    let y = dlon.sin() * lat2_rad.cos();
    let x = lat1_rad.cos() * lat2_rad.sin() - lat1_rad.sin() * lat2_rad.cos() * dlon.cos();

    let bearing = y.atan2(x).to_degrees();
    (bearing + 360.0) % 360.0
}

pub fn calculate_entry_type(start_heading: f64, inbound_course: f64, right_turns: bool) -> String {
    let relative_to_inbound = (start_heading - inbound_course + 360.0) % 360.0;
    let margin = 5.0;

    if right_turns {
        if relative_to_inbound >= (90.0 - margin) && relative_to_inbound <= (90.0 + margin) {
            "DIRECT/TEARDROP".to_string()
        } else if relative_to_inbound >= (160.0 - margin) && relative_to_inbound <= (160.0 + margin) {
            "TEARDROP/PARALLEL".to_string()
        } else if relative_to_inbound >= 270.0 || relative_to_inbound < (90.0 - margin) {
            "DIRECT".to_string()
        } else if relative_to_inbound > (90.0 + margin) && relative_to_inbound < (160.0 - margin) {
            "TEARDROP".to_string()
        } else {
            "PARALLEL".to_string()
        }
    } else {
        if relative_to_inbound >= (90.0 - margin) && relative_to_inbound <= (90.0 + margin) {
            "DIRECT/PARALLEL".to_string()
        } else if relative_to_inbound >= (200.0 - margin) && relative_to_inbound <= (200.0 + margin) {
            "PARALLEL/TEARDROP".to_string()
        } else if relative_to_inbound >= 270.0 || relative_to_inbound < (90.0 - margin) {
            "DIRECT".to_string()
        } else if relative_to_inbound > (200.0 + margin) && relative_to_inbound < 270.0 {
            "TEARDROP".to_string()
        } else {
            "PARALLEL".to_string()
        }
    }
}
