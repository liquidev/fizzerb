use std::f64::consts::PI;

use fizzerb_model::{glam::Vec2, Space};

use crate::error::Error;

use super::{context::RenderContext, transform::Transform, Color};

pub struct SpaceStyle {
    pub(crate) wall_color: Color,
    pub(crate) wall_width: f64,

    pub(crate) speaker_color: Color,
    pub(crate) speaker_size: f64,

    pub(crate) microphone_color: Color,
    pub(crate) microphone_size: f64,
    pub(crate) microphone_thickness: f64,
}

pub struct SpaceRenderer<'a> {
    pub space: &'a Space,
    pub style: &'a SpaceStyle,
    pub transform: &'a Transform,
}

impl<'a> SpaceRenderer<'a> {
    pub fn draw(self, renderer: &RenderContext) -> Result<(), Error> {
        renderer.save()?;
        self.transform.apply(renderer);

        renderer.new_path();
        for wall in &self.space.walls {
            renderer.move_to(wall.start.x as f64, wall.start.y as f64);
            renderer.line_to(wall.end.x as f64, wall.end.y as f64);
        }
        renderer.set_source_color(&self.style.wall_color);
        renderer.set_line_width(self.style.wall_width / self.transform.zoom as f64);
        renderer.stroke()?;

        renderer.new_path();
        for speaker in &self.space.speakers {
            renderer.arc(
                speaker.position.x as f64,
                speaker.position.y as f64,
                self.style.speaker_size,
                0.0,
                2.0 * PI,
            );
        }
        renderer.set_source_color(&self.style.speaker_color);
        renderer.fill()?;

        renderer.new_path();
        for microphone in &self.space.microphones {
            renderer.arc(
                microphone.position.x as f64,
                microphone.position.y as f64,
                self.style.microphone_size - self.style.microphone_thickness / 2.0,
                0.0,
                2.0 * PI,
            );
        }
        renderer.set_source_color(&self.style.microphone_color);
        renderer.set_line_width(self.style.microphone_thickness);
        renderer.stroke()?;

        renderer.restore()?;
        Ok(())
    }
}

impl Default for SpaceStyle {
    fn default() -> Self {
        Self {
            wall_color: Color::from_hex_rgb(0x071013),
            wall_width: 10.0,

            speaker_color: Color::from_hex_rgb(0xEC5740),
            speaker_size: 0.08,

            microphone_color: Color::from_hex_rgb(0x23B5D3),
            microphone_size: 0.08,
            microphone_thickness: 0.04,
        }
    }
}
