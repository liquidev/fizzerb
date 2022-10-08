use druid::{Color, Env, Key};

use super::tool;
use crate::style::color;

pub const BACKGROUND: Key<Color> = style_key!("space-editor.background");

pub const WALL_COLOR: Key<Color> = style_key!("space-editor.wall.color");
pub const WALL_THICKNESS: Key<f64> = style_key!("space-editor.wall.thickness");

pub const MICROPHONE_COLOR: Key<Color> = style_key!("space-editor.microphone.color");
pub const MICROPHONE_THICKNESS: Key<f64> = style_key!("space-editor.microphone.thickness");
pub const MICROPHONE_RADIUS: Key<f64> = style_key!("space-editor.microphone.radius");

pub const SPEAKER_COLOR: Key<Color> = style_key!("space-editor.speaker.color");
pub const SPEAKER_RADIUS: Key<f64> = style_key!("space-editor.speaker.radius");

pub fn configure_env(env: &mut Env) {
    env.set(BACKGROUND, color(0xF7F7F8));

    env.set(WALL_COLOR, color(0x071013));
    env.set(WALL_THICKNESS, 0.5);

    env.set(MICROPHONE_COLOR, color(0x23B5D3));
    env.set(MICROPHONE_RADIUS, 0.5);
    env.set(MICROPHONE_THICKNESS, 0.25);

    env.set(SPEAKER_COLOR, color(0xEC5740));
    env.set(SPEAKER_RADIUS, 0.5);

    tool::style::configure_env(env);
}
