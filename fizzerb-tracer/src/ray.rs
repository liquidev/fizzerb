//! Raycasting math.

use fizzerb_model::Wall;
use glam::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct LineSegment {
    pub a: Vec2,
    pub b: Vec2,
}

impl LineSegment {
    pub fn from_wall(wall: &Wall) -> Self {
        Self {
            a: wall.start,
            b: wall.end,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub start: Vec2,
    pub direction: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub struct RayHit {
    pub position: Vec2,
    pub ray_length: f32,
}

impl Ray {
    /// Casts the ray against a line segment.
    pub fn cast(self, segment: LineSegment) -> Option<RayHit> {
        let ray_start = self.start;
        let ray_direction = self.direction;
        let segment_start = segment.a;
        let segment_direction = segment.b - segment.a;
        if ray_direction
            .normalize()
            .abs_diff_eq(segment_direction.normalize(), 1e-9)
        {
            return None;
        }
        let t2 = (ray_direction.x * (segment_start.y - ray_start.y)
            + ray_direction.y * (ray_start.x - segment_start.x))
            / (segment_direction.x * ray_direction.y - segment_direction.y * ray_direction.x);
        let t1 = (segment_start.x + segment_direction.x * t2 - ray_start.x) / ray_direction.x;
        if t1 < 0.0 || !(0.0..=1.0).contains(&t2) {
            return None;
        }
        Some(RayHit {
            position: ray_start + ray_direction * t1,
            ray_length: t1,
        })
    }
}
