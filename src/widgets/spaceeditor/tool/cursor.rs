use druid::{
    kurbo::{Circle, Line},
    piet::{LineCap, StrokeStyle},
    Color, Env, Event, EventCtx, PaintCtx, Point, RenderContext, Vec2,
};
use spaceeditor::{
    data::{Microphone, Speaker, Wall},
    transform::Transform,
};

use super::ToolImpl;
use crate::{
    math::PointExtHitTests,
    widgets::{
        data::{EditableSpace, Object},
        spaceeditor, SpaceEditorProjectData,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Idle,
    Dragging,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HotPart {
    EntireObject,
    WallStart,
    WallEnd,
}

impl HotPart {
    fn should_move_wall_start(&self) -> bool {
        matches!(self, HotPart::EntireObject | HotPart::WallStart)
    }

    fn should_move_wall_end(&self) -> bool {
        matches!(self, HotPart::EntireObject | HotPart::WallEnd)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HotState {
    object: usize,
    part: HotPart,
}

pub struct CursorTool {
    hot_state: Option<HotState>,
    focused_state: Option<HotState>,
    state: State,
    last_mouse_pos: Point,
}

impl CursorTool {
    pub fn new() -> Self {
        Self {
            hot_state: None,
            focused_state: None,
            state: State::Idle,
            last_mouse_pos: Point::ZERO,
        }
    }

    fn make_object_hot_at_position(
        &mut self,
        object_params: &CachedObjectParams,
        space: &EditableSpace,
        position: Point,
    ) {
        self.hot_state = None;

        // Prioritize the focused object, then try other objects.
        if let Some(HotState { object: index, .. }) = self.focused_state {
            let object = &space.objects[index];
            if self.check_object_hotness(object, index, position, object_params) {
                return;
            }
        }

        for (index, object) in space.objects.iter().enumerate() {
            let did_set_hot_state =
                self.check_object_hotness(object, index, position, object_params);

            // The hotness of focused objects takes priority over non-focused objects.
            if did_set_hot_state && self.focused_state == self.hot_state {
                break;
            }
        }
    }

    fn check_object_hotness(
        &mut self,
        object: &Object,
        object_index: usize,
        position: Point,
        object_params: &CachedObjectParams,
    ) -> bool {
        match object {
            Object::Wall(wall) => {
                let hot_part = if position.in_circle(wall.start, object_params.handle_radius) {
                    Some(HotPart::WallStart)
                } else if position.in_circle(wall.end, object_params.handle_radius) {
                    Some(HotPart::WallEnd)
                } else if position.near_line(wall.start, wall.end, object_params.wall_thickness) {
                    Some(HotPart::EntireObject)
                } else {
                    None
                };
                if let Some(part) = hot_part {
                    self.hot_state = Some(HotState {
                        object: object_index,
                        part,
                    });
                    return true;
                }
            }
            Object::Microphone(microphone) => {
                let is_hot =
                    position.in_circle(microphone.position, object_params.microphone_radius);
                if is_hot {
                    self.hot_state = Some(HotState {
                        object: object_index,
                        part: HotPart::EntireObject,
                    });
                    return true;
                }
            }
            Object::Speaker(speaker) => {
                let is_hot = position.in_circle(speaker.position, object_params.speaker_radius);
                if is_hot {
                    self.hot_state = Some(HotState {
                        object: object_index,
                        part: HotPart::EntireObject,
                    });
                    return true;
                }
            }
        }
        false
    }

    fn drag_entire_object(&mut self, object: &mut Object, part: HotPart, delta: Vec2) {
        match object {
            Object::Wall(Wall { start, end, .. }) => {
                if part.should_move_wall_start() {
                    *start += delta;
                }
                if part.should_move_wall_end() {
                    *end += delta;
                }
            }
            Object::Microphone(Microphone { position })
            | Object::Speaker(Speaker { position, .. }) => *position += delta,
        }
    }

    fn focused_object_is_hot(&self) -> bool {
        self.focused_state.map(|s| s.object) == self.hot_state.map(|s| s.object)
    }

    fn object_part_is_hot(&self, object_index: usize, part: HotPart) -> bool {
        self.hot_state
            .map(|s| s.object == object_index && s.part == part)
            .unwrap_or(false)
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
        match (self.state, event) {
            (State::Idle, Event::MouseMove(mouse)) => {
                let object_params =
                    CachedObjectParams::from_env_and_transform(env, &data.transform);
                let previous_hot = self.hot_state;
                self.make_object_hot_at_position(&object_params, &data.space, mouse.pos);
                if self.hot_state != previous_hot {
                    ctx.request_paint();
                }
            }
            (State::Idle, Event::MouseDown(mouse)) => {
                if mouse.button.is_left() {
                    self.focused_state = self.hot_state;
                    if self.hot_state.is_some() {
                        self.state = State::Dragging;
                        ctx.set_active(true);
                        ctx.request_paint();
                    }
                    ctx.set_handled();
                }
            }

            (State::Dragging, Event::MouseMove(mouse)) => {
                if let Some(HotState { object, part }) = self.focused_state {
                    let object = &mut data.edit_space().objects[object];
                    let delta = mouse.pos - self.last_mouse_pos;
                    self.drag_entire_object(object, part, delta);
                    ctx.request_paint();
                }
            }

            (_, Event::MouseUp(_)) => {
                self.state = State::Idle;
                ctx.set_active(false);
                ctx.request_paint();
            }

            _ => (),
        }

        if let Event::MouseMove(mouse) = event {
            self.last_mouse_pos = mouse.pos;
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &SpaceEditorProjectData, env: &Env) {
        let viewport_size = ctx.size();
        let primary_color = env.get(style::PRIMARY_SELECTION_COLOR);
        let secondary_color = env.get(style::SECONDARY_SELECTION_COLOR);

        if let Some(HotState {
            object: object_index,
            ..
        }) = self.focused_state
        {
            let object = &data.space.objects[object_index];

            let thickness = if self.object_part_is_hot(object_index, HotPart::EntireObject) {
                env.get(style::HOT_FOCUSED_OUTLINE_THICKNESS)
            } else {
                env.get(style::FOCUSED_OUTLINE_THICKNESS)
            };
            paint_object_outline(ctx, env, &data.transform, viewport_size, object, thickness);

            if let &Object::Wall(Wall { start, end, .. }) = object {
                let start = data.transform.to_screen_space(start, viewport_size);
                let end = data.transform.to_screen_space(end, viewport_size);
                paint_object_handle(
                    ctx,
                    env,
                    start,
                    &primary_color,
                    &secondary_color,
                    self.object_part_is_hot(object_index, HotPart::WallStart),
                );
                paint_object_handle(
                    ctx,
                    env,
                    end,
                    &primary_color,
                    &secondary_color,
                    self.object_part_is_hot(object_index, HotPart::WallEnd),
                );
            }
        }

        if let Some(HotState { object, .. }) = self.hot_state {
            if !self.focused_object_is_hot() {
                let object = &data.space.objects[object];
                let thickness = env.get(style::HOT_OUTLINE_THICKNESS);
                paint_object_outline(ctx, env, &data.transform, viewport_size, object, thickness);
            }
        }
    }
}

struct CachedObjectParams {
    microphone_radius: f64,
    speaker_radius: f64,
    wall_thickness: f64,
    handle_radius: f64,
}

impl CachedObjectParams {
    fn from_env_and_transform(env: &Env, transform: &Transform) -> Self {
        Self {
            microphone_radius: env.get(spaceeditor::style::MICROPHONE_RADIUS),
            speaker_radius: env.get(spaceeditor::style::SPEAKER_RADIUS),
            wall_thickness: env.get(spaceeditor::style::WALL_THICKNESS) / transform.zoom() * 16.0,
            handle_radius: env.get(style::HOT_HANDLE_OUTER_RADIUS) / transform.zoom() * 2.0,
        }
    }
}

fn paint_object_handle(
    ctx: &mut PaintCtx,
    env: &Env,
    position: Point,
    primary_color: &Color,
    secondary_color: &Color,
    is_hot: bool,
) {
    let (inner_radius, outer_radius) = if is_hot {
        (
            env.get(style::HOT_HANDLE_INNER_RADIUS),
            env.get(style::HOT_HANDLE_OUTER_RADIUS),
        )
    } else {
        (
            env.get(style::IDLE_HANDLE_INNER_RADIUS),
            env.get(style::IDLE_HANDLE_OUTER_RADIUS),
        )
    };
    ctx.fill(Circle::new(position, outer_radius), secondary_color);
    ctx.fill(Circle::new(position, inner_radius), primary_color);
}

fn paint_object_outline(
    ctx: &mut PaintCtx,
    env: &Env,
    transform: &Transform,
    viewport_size: druid::Size,
    object: &Object,
    thickness: f64,
) {
    let primary_color = env.get(style::PRIMARY_SELECTION_COLOR);
    let stroke_style = StrokeStyle::default().line_cap(LineCap::Round);
    match object {
        Object::Wall(wall) => {
            let start = transform.to_screen_space(wall.start, viewport_size);
            let end = transform.to_screen_space(wall.end, viewport_size);
            ctx.stroke_styled(
                Line::new(start, end),
                &primary_color,
                thickness,
                &stroke_style,
            );
        }
        Object::Microphone(microphone) => {
            let position = transform.to_screen_space(microphone.position, viewport_size);
            let radius = env.get(style::MICROPHONE_RADIUS) * transform.zoom();
            ctx.stroke_styled(
                Circle::new(position, radius),
                &primary_color,
                thickness,
                &stroke_style,
            );
        }
        Object::Speaker(speaker) => {
            let position = transform.to_screen_space(speaker.position, viewport_size);
            let radius = env.get(style::SPEAKER_RADIUS) * transform.zoom();
            ctx.stroke_styled(
                Circle::new(position, radius),
                &primary_color,
                thickness,
                &stroke_style,
            );
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
    pub const HOT_FOCUSED_OUTLINE_THICKNESS: Key<f64> =
        style_key!("tool.cursor.selection.hot-focused-thickness");

    pub const MICROPHONE_RADIUS: Key<f64> = style_key!("tool.cursor.microphone.radius");
    pub const SPEAKER_RADIUS: Key<f64> = style_key!("tool.cursor.speaker.radius");

    pub const IDLE_HANDLE_INNER_RADIUS: Key<f64> =
        style_key!("tool.cursor.handle.idle.inner-radius");
    pub const IDLE_HANDLE_OUTER_RADIUS: Key<f64> =
        style_key!("tool.cursor.handle.idle.outer-radius");
    pub const HOT_HANDLE_INNER_RADIUS: Key<f64> = style_key!("tool.cursor.handle.hot.inner-radius");
    pub const HOT_HANDLE_OUTER_RADIUS: Key<f64> = style_key!("tool.cursor.handle.hot.outer-radius");

    pub fn configure_env(env: &mut Env) {
        env.set(PRIMARY_SELECTION_COLOR, color(0x168BE3));
        env.set(SECONDARY_SELECTION_COLOR, color(0xFFFFFF));

        env.set(HOT_OUTLINE_THICKNESS, 2.0);
        env.set(FOCUSED_OUTLINE_THICKNESS, 4.0);
        env.set(HOT_FOCUSED_OUTLINE_THICKNESS, 6.0);
        env.set(
            MICROPHONE_RADIUS,
            env.get(spaceeditor::style::MICROPHONE_RADIUS) * 1.4,
        );
        env.set(
            SPEAKER_RADIUS,
            env.get(spaceeditor::style::SPEAKER_RADIUS) * 1.4,
        );

        env.set(IDLE_HANDLE_INNER_RADIUS, 4.0);
        env.set(IDLE_HANDLE_OUTER_RADIUS, 6.0);
        env.set(HOT_HANDLE_INNER_RADIUS, 6.0);
        env.set(HOT_HANDLE_OUTER_RADIUS, 8.0);
    }
}
