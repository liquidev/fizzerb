//! Data types for describing spaces.

pub extern crate glam;

use glam::Vec2;

/// A wall made up of a material.
#[derive(Debug, Clone, Copy)]
pub struct Wall {
    pub start: Vec2,
    pub end: Vec2,
    pub material: MaterialIndex,
}

/// Definition of a wall material.
#[derive(Debug, Clone)]
pub struct Material {
    /// The "color" of the wall - how much sound it absorbs. Since sound doesn't have color,
    /// this is only a float coefficient.
    pub diffuse: f32,
    /// How much sound waves are scattered by the wall.
    pub roughness: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            diffuse: 1.0,
            roughness: 0.0,
        }
    }
}

/// A speaker that plays impulses.
#[derive(Debug, Clone)]
pub struct Speaker {
    pub position: Vec2,
}

/// A microphone that registers impulses from speakers.
#[derive(Debug, Clone)]
pub struct Microphone {
    pub position: Vec2,
}

/// Index of a material inside the room.
#[derive(Debug, Clone, Copy)]
pub struct MaterialIndex(pub usize);

/// A room for recording impulse responses.
#[derive(Debug, Clone, Default)]
pub struct Space {
    pub walls: Vec<Wall>,
    pub materials: Vec<Material>,
    pub speakers: Vec<Speaker>,
    pub microphones: Vec<Microphone>,
}

impl Space {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_wall(&mut self, wall: Wall) {
        self.walls.push(wall);
    }

    pub fn add_walls(&mut self, walls: impl Iterator<Item = Wall>) {
        self.walls.extend(walls);
    }

    pub fn add_material(&mut self, material: Material) -> MaterialIndex {
        let index = self.materials.len();
        self.materials.push(material);
        MaterialIndex(index)
    }

    pub fn add_speaker(&mut self, speaker: Speaker) {
        self.speakers.push(speaker);
    }

    pub fn add_microphone(&mut self, microphone: Microphone) {
        self.microphones.push(microphone);
    }
}

pub mod walls {
    use glam::vec2;

    use super::*;

    pub fn make_box(
        position: Vec2,
        size: Vec2,
        material: MaterialIndex,
    ) -> impl Iterator<Item = Wall> {
        [
            Wall {
                start: position,
                end: position + vec2(size.x, 0.0),
                material,
            },
            Wall {
                start: position + vec2(size.x, 0.0),
                end: position + size,
                material,
            },
            Wall {
                start: position + size,
                end: position + vec2(0.0, size.y),
                material,
            },
            Wall {
                start: position + vec2(0.0, size.y),
                end: position,
                material,
            },
        ]
        .into_iter()
    }
}
