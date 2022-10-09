use druid::Data;
use fizzerb_model as model;
use model::Space;
use serde::{Deserialize, Serialize};

use crate::{math::DruidExtToGlam, sparse_set::SparseSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Data, Deserialize, Serialize)]
pub struct MaterialIndex(usize);

#[derive(Debug, Clone, PartialEq, Data, Deserialize, Serialize)]
pub struct Wall {
    pub start: druid::Point,
    pub end: druid::Point,
    pub material: MaterialIndex,
}

#[derive(Debug, Clone, PartialEq, Data, Deserialize, Serialize)]
pub struct Microphone {
    pub position: druid::Point,
}

#[derive(Debug, Clone, PartialEq, Data, Deserialize, Serialize)]
pub struct Speaker {
    pub position: druid::Point,
    pub power: f32,
}

#[derive(Debug, Clone, PartialEq, Data, Deserialize, Serialize)]
pub enum Object {
    Wall(Wall),
    Microphone(Microphone),
    Speaker(Speaker),
}

#[derive(Debug, Clone, Data, Deserialize, Serialize)]
pub struct EditableSpace {
    #[data(same_fn = "PartialEq::eq")]
    pub objects: SparseSet<Object>,
}

impl EditableSpace {
    pub fn new() -> Self {
        Self {
            objects: SparseSet::new(),
        }
    }

    pub fn to_model(&self) -> Space {
        let mut space = Space::new();

        let default_material = space.add_material(model::Material {
            diffuse: 1.0,
            roughness: 0.0,
        });

        for object in &self.objects {
            match object {
                Object::Wall(wall) => {
                    space.add_wall(model::Wall {
                        start: wall.start.to_glam(),
                        end: wall.end.to_glam(),
                        material: default_material,
                    });
                }
                Object::Microphone(microphone) => {
                    space.add_microphone(model::Microphone {
                        position: microphone.position.to_glam(),
                    });
                }
                Object::Speaker(speaker) => {
                    space.add_speaker(model::Speaker {
                        position: speaker.position.to_glam(),
                        power: speaker.power,
                    });
                }
            }
        }

        space
    }
}

impl Default for EditableSpace {
    fn default() -> Self {
        Self::new()
    }
}
