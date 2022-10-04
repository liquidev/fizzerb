//! Math utilities.

use glam::vec2;

pub trait DruidVec2Ext {
    fn to_glam(self) -> glam::Vec2;
}

impl DruidVec2Ext for druid::Vec2 {
    #[inline(always)]
    fn to_glam(self) -> glam::Vec2 {
        vec2(self.x as f32, self.y as f32)
    }
}
