use glam::Vec2;

use super::context::RenderContext;

pub struct Transform {
    pub pan: Vec2,
    pub zoom: f32,
}

impl Transform {
    pub fn apply(&self, renderer: &RenderContext) {
        renderer.translate(renderer.width / 2.0, renderer.height / 2.0);
        renderer.scale(self.zoom as f64, self.zoom as f64);
    }
}
