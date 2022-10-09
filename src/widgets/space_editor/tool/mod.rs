mod cursor;
mod wall;

use druid::{Data, Env, Event, EventCtx, PaintCtx};
use serde::{Deserialize, Serialize};

use self::{cursor::CursorTool, wall::WallTool};
use super::SpaceEditorProjectData;

pub trait ToolImpl {
    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut SpaceEditorProjectData,
        env: &Env,
    );

    fn paint(&mut self, ctx: &mut PaintCtx, data: &SpaceEditorProjectData, env: &Env);
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Data, Deserialize, Serialize)]
pub enum Tool {
    #[default]
    Cursor,
    Wall,
}

impl Tool {
    pub fn get_impl(self) -> Box<dyn ToolImpl> {
        match self {
            Tool::Cursor => Box::new(CursorTool::new()),
            Tool::Wall => Box::new(WallTool::new()),
        }
    }
}

pub mod style {
    use druid::Env;

    use super::*;

    pub fn configure_env(env: &mut Env) {
        cursor::style::configure_env(env);
        wall::style::configure_env(env);
    }
}
