use druid::Data;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Data, Deserialize, Serialize)]
pub struct MaterialIndex(usize);

#[derive(Debug, Clone, PartialEq, Data, Deserialize, Serialize)]
pub struct Wall {
    pub start: druid::Vec2,
    pub end: druid::Vec2,
    pub material: MaterialIndex,
}

#[derive(Debug, Clone, PartialEq, Data, Deserialize, Serialize)]
pub struct Microphone {
    pub position: druid::Vec2,
}

#[derive(Debug, Clone, PartialEq, Data, Deserialize, Serialize)]
pub struct Speaker {
    pub position: druid::Vec2,
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
    pub objects: Vec<Object>,
}

impl EditableSpace {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }
}
