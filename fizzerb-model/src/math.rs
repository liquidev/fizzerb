use glam::Vec2;

/// Reflects a vector along a normal.
pub fn reflect(d: Vec2, n: Vec2) -> Vec2 {
    d - 2.0 * d.dot(n) * n
}
