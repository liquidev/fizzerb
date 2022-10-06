//! Project file format.

use std::sync::Arc;

use druid::{Data, Lens, Vec2};
use serde::{Deserialize, Serialize};

use crate::{
    rendering::RenderSettings,
    widgets::{data::EditableSpace, SpaceEditorData, Transform},
};

#[derive(Clone, Data, Lens, Deserialize, Serialize)]
pub struct Project {
    pub render_settings: RenderSettings,
    pub space_editor: SpaceEditorData,
}

impl Project {
    pub fn new() -> Self {
        Self {
            render_settings: RenderSettings::default(),
            space_editor: SpaceEditorData {
                space: Arc::new(EditableSpace::new()),
                transform: Transform {
                    pan: Vec2::new(0.0, 0.0),
                    zoom: 64.0,
                },
            },
        }
    }
}

impl Default for Project {
    fn default() -> Self {
        Self::new()
    }
}
