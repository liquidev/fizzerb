use fizzerb_tracer::{RayPurpose, RecordedRay, Recording};

use crate::error::Error;

use super::{context::RenderContext, transform::Transform, Color};

pub struct RecordingStyle {
    pub ray_scale: f64,
    pub hit_point_size: f64,
    pub bounce_ray_color: Color,
    pub trace_ray_color: Color,
}

pub struct RecordingRenderer<'a> {
    pub recording: &'a Recording,
    pub transform: &'a Transform,
    pub style: &'a RecordingStyle,
}

impl<'a> RecordingRenderer<'a> {
    pub fn draw(self, renderer: &RenderContext) -> Result<(), Error> {
        renderer.save()?;
        self.transform.apply(renderer);

        renderer.set_line_width(self.style.ray_scale);
        for recorded_ray in &self.recording.rays {
            let RecordedRay { purpose, ray, hit } = recorded_ray;

            renderer.new_path();
            renderer.move_to(ray.start.x as f64, ray.start.y as f64);
            renderer.line_to(hit.position.x as f64, hit.position.y as f64);

            match purpose {
                RayPurpose::Bounce => renderer.set_source_color(&self.style.bounce_ray_color),
                RayPurpose::Trace => renderer.set_source_color(&self.style.trace_ray_color),
            };
            renderer.stroke()?;

            if purpose.is_bounce() {
                renderer.new_path();
                renderer.arc(
                    hit.position.x as f64,
                    hit.position.y as f64,
                    self.style.hit_point_size,
                    0.0,
                    2.0 * std::f64::consts::PI,
                );
                renderer.fill()?;
            }
        }

        renderer.restore()?;

        Ok(())
    }
}

impl Default for RecordingStyle {
    fn default() -> Self {
        Self {
            ray_scale: 0.02,
            hit_point_size: 0.05,
            bounce_ray_color: Color::from_hex_rgb(0xA2AEBB),
            trace_ray_color: Color::from_hex_rgb(0xEC5740),
        }
    }
}
