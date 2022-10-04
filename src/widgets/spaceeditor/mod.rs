pub mod data;
pub mod style;

use druid::{
    kurbo::{Circle, Line},
    Affine, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, RenderContext, Size, UpdateCtx, Widget,
};

use self::data::{EditableSpace, Object};

#[derive(Clone, Data)]
pub struct Transform {
    pub pan: druid::Vec2,
    pub zoom: f64,
}

#[derive(Clone, Data)]
pub struct SpaceEditorData {
    space: EditableSpace,
    transform: Transform,
}

impl SpaceEditorData {
    pub fn new(space: EditableSpace) -> Self {
        Self {
            space,
            transform: Transform {
                pan: druid::Vec2::new(0.0, 0.0),
                zoom: 25.0,
            },
        }
    }
}

pub struct SpaceEditor {}

impl SpaceEditor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Widget<SpaceEditorData> for SpaceEditor {
    fn event(
        &mut self,
        _ctx: &mut EventCtx,
        _event: &Event,
        _data: &mut SpaceEditorData,
        _env: &Env,
    ) {
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &SpaceEditorData,
        _env: &Env,
    ) {
    }

    fn update(
        &mut self,
        _ctx: &mut UpdateCtx,
        _old_data: &SpaceEditorData,
        _data: &SpaceEditorData,
        _env: &Env,
    ) {
    }

    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &SpaceEditorData,
        _env: &Env,
    ) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &SpaceEditorData, env: &Env) {
        let bounds = ctx.size().to_rect();

        ctx.fill(bounds, &env.get(style::BACKGROUND));

        ctx.save().unwrap();
        ctx.transform(Affine::translate(bounds.center().to_vec2()));
        ctx.transform(Affine::scale(data.transform.zoom));

        for object in &data.space.objects {
            match object {
                Object::Wall(wall) => {
                    ctx.stroke(
                        Line::new(wall.start.to_point(), wall.end.to_point()),
                        &env.get(style::WALL_COLOR),
                        env.get(style::WALL_THICKNESS),
                    );
                }
                Object::Microphone(microphone) => {
                    let thickness = env.get(style::MICROPHONE_THICKNESS);
                    let radius = env.get(style::MICROPHONE_RADIUS) - thickness * 0.5;
                    ctx.stroke(
                        Circle::new(microphone.position.to_point(), radius),
                        &env.get(style::MICROPHONE_COLOR),
                        thickness,
                    );
                }
                Object::Speaker(speaker) => ctx.fill(
                    Circle::new(speaker.position.to_point(), env.get(style::SPEAKER_RADIUS)),
                    &env.get(style::SPEAKER_COLOR),
                ),
            }
        }

        ctx.restore().unwrap();
    }
}
