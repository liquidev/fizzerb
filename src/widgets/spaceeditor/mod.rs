pub mod data;
pub mod style;
pub mod tool;
pub mod transform;

use std::sync::Arc;

use druid::{
    kurbo::{Circle, Line},
    piet::{LineCap, StrokeStyle},
    Affine, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, RenderContext, Size, UpdateCtx, Widget,
};
use serde::{Deserialize, Serialize};

use self::{
    data::{EditableSpace, Object},
    tool::{Tool, ToolImpl},
    transform::Transform,
};

#[derive(Clone, Data, Deserialize, Serialize)]
pub struct SpaceEditorProjectData {
    pub space: Arc<EditableSpace>,
    pub transform: Transform,
    pub tool: Tool,
}

impl SpaceEditorProjectData {
    pub fn new(space: EditableSpace) -> Self {
        Self {
            space: Arc::new(space),
            transform: Transform {
                pan: druid::Vec2::new(0.0, 0.0),
                zoom_level: 1.0,
            },
            tool: Tool::Cursor,
        }
    }

    pub fn edit_space(&mut self) -> &mut EditableSpace {
        Arc::make_mut(&mut self.space)
    }
}

pub struct SpaceEditor {
    tool: Box<dyn ToolImpl>,
}

impl SpaceEditor {
    pub fn new() -> Self {
        Self {
            tool: Tool::default().get_impl(),
        }
    }

    fn update_tool(&mut self, tool: Tool) {
        self.tool = tool.get_impl();
    }
}

impl Widget<SpaceEditorProjectData> for SpaceEditor {
    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut SpaceEditorProjectData,
        env: &Env,
    ) {
        let viewport_size = ctx.size();
        let viewport_space_event = data.transform.mouse_to_viewport_space(event, viewport_size);
        self.tool.event(ctx, &viewport_space_event, data, env);
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &SpaceEditorProjectData,
        _env: &Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            self.update_tool(data.tool);
        }
    }

    fn update(
        &mut self,
        _ctx: &mut UpdateCtx,
        old_data: &SpaceEditorProjectData,
        data: &SpaceEditorProjectData,
        _env: &Env,
    ) {
        if data.tool != old_data.tool {
            self.update_tool(data.tool);
        }
    }

    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &SpaceEditorProjectData,
        _env: &Env,
    ) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &SpaceEditorProjectData, env: &Env) {
        let bounds = ctx.size().to_rect();

        ctx.fill(bounds, &env.get(style::BACKGROUND));

        ctx.with_save(|ctx| {
            ctx.transform(Affine::translate(bounds.center().to_vec2()));
            ctx.transform(Affine::scale(data.transform.zoom()));

            for object in &data.space.objects {
                match object {
                    Object::Wall(wall) => {
                        ctx.stroke_styled(
                            Line::new(wall.start, wall.end),
                            &env.get(style::WALL_COLOR),
                            env.get(style::WALL_THICKNESS),
                            &StrokeStyle::default().line_cap(LineCap::Round),
                        );
                    }
                    Object::Microphone(microphone) => {
                        let thickness = env.get(style::MICROPHONE_THICKNESS);
                        let radius = env.get(style::MICROPHONE_RADIUS) - thickness * 0.5;
                        ctx.stroke(
                            Circle::new(microphone.position, radius),
                            &env.get(style::MICROPHONE_COLOR),
                            thickness,
                        );
                    }
                    Object::Speaker(speaker) => ctx.fill(
                        Circle::new(speaker.position, env.get(style::SPEAKER_RADIUS)),
                        &env.get(style::SPEAKER_COLOR),
                    ),
                }
            }
        });

        self.tool.paint(ctx, data, env);
    }
}
