use std::ops::Deref;

use crate::error::Error;

use super::Color;

/// Rendering backend data.
///
/// For now this is hardcoded to a Cairo surface.
pub struct RenderContext {
    pub surface: cairo::ImageSurface,
    pub cairo: cairo::Context,
}

impl RenderContext {
    pub unsafe fn new(data: *mut u8, width: u32, height: u32) -> Result<Self, Error> {
        let surface = Self::create_cairo_image_surface(data, width, height)?;
        let cairo = cairo::Context::new(&surface)?;
        Ok(Self { surface, cairo })
    }

    fn create_context(&self) -> Result<cairo::Context, Error> {
        Ok(cairo::Context::new(&self.surface)?)
    }

    pub unsafe fn resize(&mut self, data: *mut u8, width: u32, height: u32) -> Result<(), Error> {
        self.surface = Self::create_cairo_image_surface(data, width, height)?;
        self.cairo = self.create_context()?;
        Ok(())
    }

    pub fn set_source_color(&self, color: &Color) {
        self.cairo
            .set_source_rgba(color.r, color.g, color.b, color.a);
    }

    unsafe fn create_cairo_image_surface(
        data: *mut u8,
        width: u32,
        height: u32,
    ) -> Result<cairo::ImageSurface, Error> {
        Ok(cairo::ImageSurface::create_for_data_unsafe(
            data,
            cairo::Format::ARgb32,
            width as i32,
            height as i32,
            width as i32 * 4,
        )?)
    }
}

impl Deref for RenderContext {
    type Target = cairo::Context;

    fn deref(&self) -> &Self::Target {
        &self.cairo
    }
}
