//! Project file format.

use std::sync::Arc;

use druid::{Data, Lens, Vec2};
use serde::{Deserialize, Serialize};

use crate::{
    rendering::RenderSettings,
    widgets::{data::EditableSpace, tool::Tool, transform::Transform, SpaceEditorProjectData},
};

#[derive(Clone, Data, Lens, Deserialize, Serialize)]
pub struct Project {
    pub render_settings: RenderSettings,
    pub space_editor: SpaceEditorProjectData,
}

impl Project {
    pub fn new() -> Self {
        Self {
            render_settings: RenderSettings::default(),
            space_editor: SpaceEditorProjectData {
                space: Arc::new(EditableSpace::new()),
                transform: Transform {
                    pan: Vec2::new(0.0, 0.0),
                    zoom_level: 24.0,
                },
                tool: Tool::Cursor,
            },
        }
    }
}

impl Default for Project {
    fn default() -> Self {
        Self::new()
    }
}
