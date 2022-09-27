use fizzerb_model::Space;

use crate::error::Error;

use super::{context::RenderContext, Color};

pub struct SpaceStyle {
    pub(crate) walls: Color,
    pub(crate) wall_width: f64,
}

pub fn draw_space(
    renderer: &RenderContext,
    space: &Space,
    style: &SpaceStyle,
) -> Result<(), Error> {
    renderer.new_path();
    for wall in &space.walls {
        renderer.move_to(wall.start.x as f64, wall.start.y as f64);
        renderer.line_to(wall.end.x as f64, wall.end.y as f64);
    }
    renderer.set_source_color(&style.walls);
    renderer.set_line_width(style.wall_width);
    renderer.stroke()?;

    Ok(())
}

impl Default for SpaceStyle {
    fn default() -> Self {
        Self {
            walls: Color::from_hex_rgb(0x071013),
            wall_width: 0.02,
        }
    }
}
