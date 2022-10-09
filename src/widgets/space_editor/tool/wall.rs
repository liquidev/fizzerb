use druid::{
    kurbo::{Circle, Line},
    Cursor, Env, Event, EventCtx, PaintCtx, Point, RenderContext,
};

use super::ToolImpl;
use crate::widgets::{
    data::{MaterialIndex, Object, Wall},
    SpaceEditorProjectData,
};

#[derive(Debug, Clone, Copy)]
enum State {
    PlaceStart,
    PlaceEnd { start: Point },
}

pub struct WallTool {
    state: State,
    mouse_pos: Point,
}

impl WallTool {
    pub fn new() -> Self {
        Self {
            state: State::PlaceStart,
            mouse_pos: Point::ZERO,
        }
    }
}

impl ToolImpl for WallTool {
    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut SpaceEditorProjectData,
        _env: &Env,
    ) {
        ctx.set_cursor(&Cursor::Crosshair);

        match (&self.state, event) {
            (State::PlaceStart, Event::MouseDown(mouse)) => {
                self.state = State::PlaceEnd { start: mouse.pos };
                ctx.set_active(true);
            }
            (&State::PlaceEnd { start }, Event::MouseUp(mouse)) => {
                let end = mouse.pos;
                data.edit_space().objects.insert(Object::Wall(Wall {
                    start,
                    end,
                    material: MaterialIndex::default(),
                }));
                ctx.request_paint();
                ctx.set_active(true);
                self.state = State::PlaceStart;
            }
            (_, Event::MouseMove(mouse)) => {
                self.mouse_pos = mouse.pos;
                ctx.request_paint();
            }
            _ => (),
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &SpaceEditorProjectData, env: &Env) {
        let viewport_size = ctx.size();
        let end = data
            .transform
            .to_screen_space(self.mouse_pos, viewport_size);
        match &self.state {
            State::PlaceStart => {
                paint_vertex(ctx, end, env);
            }
            &State::PlaceEnd { start } => {
                let start = data.transform.to_screen_space(start, viewport_size);
                ctx.stroke(
                    Line::new(start, end),
                    &env.get(style::PRIMARY_COLOR),
                    env.get(style::LINE_THICKNESS),
                );
                paint_vertex(ctx, start, env);
                paint_vertex(ctx, end, env);
            }
        }
    }
}

fn paint_vertex(ctx: &mut PaintCtx, vertex: Point, env: &Env) {
    ctx.fill(
        Circle::new(vertex, env.get(style::VERTEX_OUTER_RADIUS)),
        &env.get(style::PRIMARY_COLOR),
    );
    ctx.fill(
        Circle::new(vertex, env.get(style::VERTEX_INNER_RADIUS)),
        &env.get(style::SECONDARY_COLOR),
    );
}

pub mod style {
    use druid::{Color, Env, Key};

    use crate::style::color;

    pub const PRIMARY_COLOR: Key<Color> = style_key!("tool.wall.primary-color");
    pub const SECONDARY_COLOR: Key<Color> = style_key!("tool.wall.secondary-color");

    pub const VERTEX_OUTER_RADIUS: Key<f64> = style_key!("took.wall.vertex.outer-radius");
    pub const VERTEX_INNER_RADIUS: Key<f64> = style_key!("took.wall.vertex.inner-radius");
    pub const LINE_THICKNESS: Key<f64> = style_key!("took.wall.line-thickness");

    pub fn configure_env(env: &mut Env) {
        env.set(PRIMARY_COLOR, color(0x49D46E));
        env.set(SECONDARY_COLOR, color(0xFFFFFF));

        env.set(VERTEX_OUTER_RADIUS, 12.0);
        env.set(VERTEX_INNER_RADIUS, 8.0);
        env.set(LINE_THICKNESS, 4.0);
    }
}
