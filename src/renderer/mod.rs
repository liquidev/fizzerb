pub mod context;
pub mod shuffler;
pub mod space;

#[derive(Debug, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f64 / 255.0,
            g: g as f64 / 255.0,
            b: b as f64 / 255.0,
            a: a as f64 / 255.0,
        }
    }

    pub fn from_hex_rgb(rgb: u32) -> Self {
        Self::from_rgba(
            ((rgb & 0xFF0000) >> 16) as u8,
            ((rgb & 0x00FF00) >> 8) as u8,
            (rgb & 0x0000FF) as u8,
            255,
        )
    }
}
