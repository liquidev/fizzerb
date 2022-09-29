use crate::{ray::Ray, RayHit, WallHit};

/// An impulse response.
#[derive(Debug, Clone)]
pub struct Response {
    pub time: f32,
    pub loudness: f32,
}

/// What a ray is for.
#[derive(Debug, Clone, Copy)]
pub enum RayPurpose {
    /// Used for generating bounces against walls.
    Bounce,
    /// Used for tracing back to the speaker.
    Trace,
}

impl RayPurpose {
    /// Returns `true` if the ray purpose is [`Bounce`].
    ///
    /// [`Bounce`]: RayPurpose::Bounce
    #[must_use]
    pub fn is_bounce(&self) -> bool {
        matches!(self, Self::Bounce)
    }
}

#[derive(Debug, Clone)]
pub struct RecordedRay {
    pub purpose: RayPurpose,
    pub ray: Ray,
    pub hit: RayHit,
}

/// Recording of impulse responses.
#[derive(Debug, Clone)]
pub struct Recording {
    pub responses: Vec<Response>,
    pub rays: Vec<RecordedRay>,
}
