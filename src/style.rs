//! Styling keys.

use druid::{Color, Env, FontDescriptor, FontFamily, FontStyle, FontWeight, Insets, Key};

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

pub const TEXT: Key<FontDescriptor> = style_key!("text");

pub const WINDOW_PADDING: Key<Insets> = style_key!("window-padding");

pub fn configure_env(env: &mut Env) -> Result<(), Error> {
    env.set(druid::theme::TEXT_COLOR, color(0x071013));

    env.set(
        TEXT,
        FontDescriptor {
            family: FontFamily::SYSTEM_UI,
            size: 13.0,
            weight: FontWeight::MEDIUM,
            style: FontStyle::Regular,
        },
    );

    env.set(WINDOW_PADDING, 16.0);

    widgets::button::style::configure_env(env);
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
