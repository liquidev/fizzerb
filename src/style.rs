//! Styling keys.

use druid::{Color, Env, FontDescriptor, FontFamily, FontStyle, FontWeight, Key};

use crate::{error::Error, widgets};

macro_rules! key {
    ($key:expr) => {
        Key::new(concat!("net.liquidev.fizzerb.", $key))
    };
}

macro_rules! style_key {
    ($key:expr) => {
        key!(concat!("style.", $key))
    };
}

pub const FONT_REGULAR: Key<FontDescriptor> = style_key!("font.regular");

pub fn configure_env(env: &mut Env) -> Result<(), Error> {
    env.set(
        FONT_REGULAR,
        FontDescriptor {
            family: FontFamily::SANS_SERIF,
            size: 14.0,
            weight: FontWeight::REGULAR,
            style: FontStyle::Regular,
        },
    );

    widgets::spaceeditor::style::configure_env(env);

    Ok(())
}

pub fn color(rgb: u32) -> Color {
    Color::rgba8(
        ((rgb & 0xFF0000) >> 16) as u8,
        ((rgb & 0x00FF00) >> 8) as u8,
        (rgb & 0x0000FF) as u8,
        255,
    )
}
