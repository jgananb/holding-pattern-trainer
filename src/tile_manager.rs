use eframe::egui;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TileCoord {
    pub zoom: u8,
    pub x: u32,
    pub y: u32,
}

pub struct TileManager {
    cache_dir: PathBuf,
    pub tiles: Arc<Mutex<HashMap<TileCoord, Option<egui::TextureHandle>>>>,
}

impl TileManager {
    pub fn new() -> Self {
        let cache_dir = PathBuf::from("map_cache");
        fs::create_dir_all(&cache_dir).ok();

        Self {
            cache_dir,
            tiles: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn lat_lon_to_tile(lat: f64, lon: f64, zoom: u8) -> (u32, u32) {
        let n = 2_f64.powi(zoom as i32);
        let x = ((lon + 180.0) / 360.0 * n).floor() as u32;
        let y = ((1.0 - (lat.to_radians().tan() + 1.0 / lat.to_radians().cos()).ln() / std::f64::consts::PI) / 2.0 * n).floor() as u32;
        (x, y)
    }

    pub fn tile_to_lat_lon(x: u32, y: u32, zoom: u8) -> (f64, f64) {
        let n = 2_f64.powi(zoom as i32);
        let lon = x as f64 / n * 360.0 - 180.0;
        let lat_rad = ((1.0 - 2.0 * y as f64 / n) * std::f64::consts::PI).sinh().atan();
        let lat = lat_rad.to_degrees();
        (lat, lon)
    }

    pub fn get_or_load_tile(&self, coord: TileCoord, ctx: &egui::Context) -> Option<egui::TextureHandle> {
        let mut tiles = self.tiles.lock().unwrap();

        if let Some(maybe_texture) = tiles.get(&coord) {
            return maybe_texture.clone();
        }

        let coord_clone = coord;
        let cache_dir = self.cache_dir.clone();
        let tiles_clone = self.tiles.clone();
        let ctx_clone = ctx.clone();

        std::thread::spawn(move || {
            let tile_path = cache_dir.join(format!("{}_{}_{}.png", coord_clone.zoom, coord_clone.x, coord_clone.y));

            let path = if tile_path.exists() {
                Some(tile_path)
            } else {
                let url = format!(
                    "https://tile.openstreetmap.org/{}/{}/{}.png",
                    coord_clone.zoom, coord_clone.x, coord_clone.y
                );

                if let Ok(client) = reqwest::blocking::Client::builder()
                    .user_agent("Holding Trainer/0.1")
                    .timeout(Duration::from_secs(5))
                    .build()
                {
                    if let Ok(response) = client.get(&url).send() {
                        if response.status().is_success() {
                            if let Ok(bytes) = response.bytes() {
                                if fs::write(&tile_path, &bytes).is_ok() {
                                    std::thread::sleep(Duration::from_millis(100));
                                    Some(tile_path)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            };

            if let Some(p) = path {
                if let Ok(img) = image::open(&p) {
                    let img = img.to_rgba8();
                    let size = [img.width() as usize, img.height() as usize];
                    let pixels = img.into_raw();
                    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &pixels);

                    let texture = ctx_clone.load_texture(
                        format!("tile_{}_{}_{}", coord_clone.zoom, coord_clone.x, coord_clone.y),
                        color_image,
                        Default::default()
                    );

                    if let Ok(mut tiles) = tiles_clone.lock() {
                        tiles.insert(coord_clone, Some(texture));
                        ctx_clone.request_repaint();
                    }
                }
            } else {
                if let Ok(mut tiles) = tiles_clone.lock() {
                    tiles.insert(coord_clone, None);
                }
            }
        });

        tiles.insert(coord, None);
        None
    }
}
