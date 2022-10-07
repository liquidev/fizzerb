use druid::{Data, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Clone, Data, Deserialize, Serialize)]
pub struct Transform {
    pub pan: Vec2,
    pub zoom_level: f64,
}

impl Transform {
    /// Returns the zoom amount, calculated from the zoom level.
    pub fn zoom(&self) -> f64 {
        f64::powf(2.0, self.zoom_level * 0.25)
    }
}
