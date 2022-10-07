use druid::{
    kurbo::{Circle, Line},
    piet::{LineCap, StrokeStyle},
    Env, Event, EventCtx, PaintCtx, Point, RenderContext,
};

use super::ToolImpl;
use crate::{
    math::PointExtHitTests,
    widgets::{
        data::{EditableSpace, Object},
        spaceeditor, SpaceEditorProjectData,
    },
};

pub struct CursorTool {
    hot_object: Option<usize>,
    focused_object: Option<usize>,
}

impl CursorTool {
    pub fn new() -> Self {
        Self {
            hot_object: None,
            focused_object: None,
        }
    }

    fn make_object_hot_at_position(
        &mut self,
        object_params: &CachedObjectParams,
        space: &EditableSpace,
        position: Point,
    ) {
        self.hot_object = None;
        for (index, object) in space.objects.iter().enumerate() {
            match object {
                Object::Wall(wall) => {
                    let is_hot =
                        position.near_line(wall.start, wall.end, object_params.wall_thickness)
                            || position.in_circle(wall.start, object_params.wall_thickness)
                            || position.in_circle(wall.end, object_params.wall_thickness);
                    if is_hot {
                        self.hot_object = Some(index);
                        break;
                    }
                }
                Object::Microphone(microphone) => {
                    let is_hot =
                        position.in_circle(microphone.position, object_params.microphone_radius);
                    if is_hot {
                        self.hot_object = Some(index);
                        break;
                    }
                }
                Object::Speaker(speaker) => {
                    let is_hot = position.in_circle(speaker.position, object_params.speaker_radius);
                    if is_hot {
                        self.hot_object = Some(index);
                        break;
                    }
                }
            }
        }
    }
}

impl ToolImpl for CursorTool {
    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut SpaceEditorProjectData,
        env: &Env,
    ) {
        if let Event::MouseMove(event) = event {
            let object_params = CachedObjectParams::from_env(env);
            let previous_hot = self.hot_object;
            self.make_object_hot_at_position(&object_params, &data.space, event.pos);
            if self.hot_object != previous_hot {
                ctx.request_paint();
            }
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &SpaceEditorProjectData, env: &Env) {
        let primary_color = env.get(style::PRIMARY_SELECTION_COLOR);
        let viewport_size = ctx.size();

        if let Some(hot_index) = self.hot_object {
            let object = &data.space.objects[hot_index];
            let thickness = env.get(style::HOT_OUTLINE_THICKNESS);
            let stroke_style = StrokeStyle::default().line_cap(LineCap::Round);
            match object {
                Object::Wall(wall) => {
                    let start = data.transform.to_screen_space(wall.start, viewport_size);
                    let end = data.transform.to_screen_space(wall.end, viewport_size);
                    ctx.stroke_styled(
                        Line::new(start, end),
                        &primary_color,
                        thickness,
                        &stroke_style,
                    );
                }
                Object::Microphone(microphone) => {
                    let position = data
                        .transform
                        .to_screen_space(microphone.position, viewport_size);
                    let radius = env.get(style::MICROPHONE_RADIUS) * data.transform.zoom();
                    ctx.stroke_styled(
                        Circle::new(position, radius),
                        &primary_color,
                        thickness,
                        &stroke_style,
                    );
                }
                Object::Speaker(speaker) => {
                    let position = data
                        .transform
                        .to_screen_space(speaker.position, viewport_size);
                    let radius = env.get(style::SPEAKER_RADIUS) * data.transform.zoom();
                    ctx.stroke_styled(
                        Circle::new(position, radius),
                        &primary_color,
                        thickness,
                        &stroke_style,
                    );
                }
            }
        }
    }
}

struct CachedObjectParams {
    microphone_radius: f64,
    speaker_radius: f64,
    wall_thickness: f64,
}

impl CachedObjectParams {
    fn from_env(env: &Env) -> Self {
        Self {
            microphone_radius: env.get(spaceeditor::style::MICROPHONE_RADIUS),
            speaker_radius: env.get(spaceeditor::style::SPEAKER_RADIUS),
            wall_thickness: env.get(spaceeditor::style::WALL_THICKNESS),
        }
    }
}

pub mod style {
    use druid::{Color, Env, Key};

    use crate::{style::color, widgets::spaceeditor};

    pub const PRIMARY_SELECTION_COLOR: Key<Color> =
        style_key!("tool.cursor.selection.primary-color");
    pub const SECONDARY_SELECTION_COLOR: Key<Color> =
        style_key!("tool.cursor.selection.secondary-color");

    pub const HOT_OUTLINE_THICKNESS: Key<f64> = style_key!("tool.cursor.selection.hot-thickness");
    pub const FOCUSED_OUTLINE_THICKNESS: Key<f64> =
        style_key!("tool.cursor.selection.focused-thickness");

    pub const MICROPHONE_RADIUS: Key<f64> = style_key!("tool.cursor.microphone.radius");
    pub const SPEAKER_RADIUS: Key<f64> = style_key!("tool.cursor.speaker.radius");

    pub fn configure_env(env: &mut Env) {
        env.set(PRIMARY_SELECTION_COLOR, color(0x168BE3));
        env.set(SECONDARY_SELECTION_COLOR, color(0xFFFFFF));

        env.set(HOT_OUTLINE_THICKNESS, 3.0);
        env.set(FOCUSED_OUTLINE_THICKNESS, 6.0);
        env.set(
            MICROPHONE_RADIUS,
            env.get(spaceeditor::style::MICROPHONE_RADIUS) * 1.4,
        );
        env.set(
            SPEAKER_RADIUS,
            env.get(spaceeditor::style::SPEAKER_RADIUS) * 1.4,
        );
    }
}
