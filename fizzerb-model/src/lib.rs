//! Data types for describing spaces.

pub mod math;

pub extern crate glam;

use glam::{vec2, Vec2};

/// An impulse response traced from a single bounce.
#[derive(Debug, Clone, Copy)]
pub struct Response {
    pub time: f32,
    pub loudness: f32,
    pub bounces: usize,
}

/// A wall made up of a material.
#[derive(Debug, Clone, Copy)]
pub struct Wall {
    pub start: Vec2,
    pub end: Vec2,
    pub material: MaterialIndex,
}

impl Wall {
    /// Returns the normal vector of this wall.
    pub fn normal(&self) -> Vec2 {
        let direction = (self.end - self.start).normalize();
        vec2(-direction.y, direction.x)
    }

    /// Reflects a ray facing the given `direction` against this wall and returns the reflected ray.
    pub fn reflect(&self, direction: Vec2) -> Vec2 {
        let normal = self.normal();
        math::reflect(direction, normal)
    }
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
    pub power: f32,
}

/// A microphone that registers impulses from speakers.
#[derive(Debug, Clone)]
pub struct Microphone {
    pub position: Vec2,
}

/// Index of a wall inside the room.
#[derive(Debug, Clone, Copy)]
pub struct WallIndex(pub usize);

/// Index of a speaker inside the room.
#[derive(Debug, Clone, Copy)]
pub struct SpeakerIndex(pub usize);

/// Index of a microphone inside the room.
#[derive(Debug, Clone, Copy)]
pub struct MicrophoneIndex(pub usize);

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

    pub fn add_wall(&mut self, wall: Wall) -> WallIndex {
        let index = self.walls.len();
        self.walls.push(wall);
        WallIndex(index)
    }

    pub fn add_walls(&mut self, walls: impl Iterator<Item = Wall>) {
        self.walls.extend(walls);
    }

    pub fn add_material(&mut self, material: Material) -> MaterialIndex {
        let index = self.materials.len();
        self.materials.push(material);
        MaterialIndex(index)
    }

    pub fn add_speaker(&mut self, speaker: Speaker) -> SpeakerIndex {
        let index = self.speakers.len();
        self.speakers.push(speaker);
        SpeakerIndex(index)
    }

    pub fn add_microphone(&mut self, microphone: Microphone) -> MicrophoneIndex {
        let index = self.microphones.len();
        self.microphones.push(microphone);
        MicrophoneIndex(index)
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
