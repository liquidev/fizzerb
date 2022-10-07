//! Math utilities.

use druid::Point;
use glam::vec2;

pub trait DruidExtToGlam {
    fn to_glam(self) -> glam::Vec2;
}

impl DruidExtToGlam for druid::Vec2 {
    #[inline(always)]
    fn to_glam(self) -> glam::Vec2 {
        vec2(self.x as f32, self.y as f32)
    }
}

impl DruidExtToGlam for druid::Point {
    #[inline(always)]
    fn to_glam(self) -> glam::Vec2 {
        vec2(self.x as f32, self.y as f32)
    }
}

pub trait PointExtHitTests {
    fn in_circle(self, center: Point, radius: f64) -> bool;

    fn distance_to_line_squared(self, a: Point, b: Point) -> f64;
    fn near_line(self, a: Point, b: Point, distance: f64) -> bool;
}

impl PointExtHitTests for Point {
    fn in_circle(self, center: Point, radius: f64) -> bool {
        self.distance_squared(center) <= radius * radius
    }

    fn distance_to_line_squared(self, a: Point, b: Point) -> f64 {
        let l2 = a.distance_squared(b);
        if l2.abs() < 1e-9 {
            self.distance_squared(a)
        } else {
            let t = ((self - a).dot(b - a) / l2).clamp(0.0, 1.0);
            let projection = a + t * (b - a);
            self.distance_squared(projection)
        }
    }

    fn near_line(self, a: Point, b: Point, radius: f64) -> bool {
        self.distance_to_line_squared(a, b) <= radius * radius
    }
}
