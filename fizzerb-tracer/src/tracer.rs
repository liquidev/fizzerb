use std::time::Instant;

use fizzerb_model::{MicrophoneIndex, Space, Speaker, SpeakerIndex, WallIndex};
use glam::Vec2;

use crate::{
    ray::{LineSegment, Ray, RayHit},
    RayPurpose, RecordedRay, Recording,
};

pub const SPEED_OF_SOUND_IN_AIR: f32 = 343.0;

#[derive(Debug, Clone)]
pub struct TracerConfig {
    /// The speed of sound in m/s.
    pub speed_of_sound: f32,

    /// The maximal number of times a traced ray can bounce off of walls.
    pub max_bounces: usize,

    /// Whether to record casted rays into the recording. Disabling this may improve performance.
    pub record_rays: bool,
}

/// Raytracer state.
#[derive(Debug, Clone)]
pub struct Tracer<'r> {
    pub space: &'r Space,
    pub config: &'r TracerConfig,
}

impl<'r> Tracer<'r> {
    pub fn new(space: &'r Space, config: &'r TracerConfig) -> Self {
        Self { space, config }
    }

    /// Traces a single ray for a microphone-speaker pair.
    ///
    /// `start_ray` is assumed to be normalized.
    pub fn perform_trace(
        &mut self,
        microphone_index: MicrophoneIndex,
        speaker_index: SpeakerIndex,
        start_ray: Vec2,
    ) -> Recording {
        let start = Instant::now();

        let microphone = &self.space.microphones[microphone_index.0];
        let speaker = &self.space.speakers[speaker_index.0];

        log::debug!("tracing from {microphone_index:?} to {speaker_index:?}");

        let mut recorded_rays = vec![];
        let mut ray = Ray {
            start: microphone.position,
            direction: start_ray,
        };
        let mut distance_bounced = 0.0_f32;
        for _i in 0..(self.config.max_bounces + 1) {
            if let Some(hit) = trace_to_walls(ray, self.space) {
                recorded_rays.push(RecordedRay {
                    purpose: RayPurpose::Bounce,
                    ray,
                    hit: hit.ray,
                });

                let wall = &self.space.walls[hit.wall.0];
                let reflected = wall.reflect(ray.direction);
                ray = Ray {
                    start: hit.ray.position + reflected * 0.001,
                    direction: reflected,
                };
                distance_bounced += hit.ray.ray_length;

                if let Some(trace) = trace_to_speaker(ray.start, self.space, speaker) {
                    recorded_rays.push(RecordedRay {
                        purpose: RayPurpose::Trace,
                        ray: trace.ray,
                        hit: RayHit {
                            position: speaker.position,
                            ray_length: trace.distance_to_speaker,
                        },
                    });
                }
            } else {
                log::trace!("empty space hit, finishing off");
                break;
            }
        }

        let end = Instant::now();
        log::debug!("tracing took {:?}", end - start);

        Recording {
            responses: vec![],
            rays: recorded_rays,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WallHit {
    pub ray: RayHit,
    pub wall: WallIndex,
}

/// Traces the ray against all walls within the space, and returns the wall that was hit
/// (if any.)
fn trace_to_walls(ray: Ray, space: &Space) -> Option<WallHit> {
    // Kind of naive approach but good code is outside my budget atm
    let mut closest_hit: Option<RayHit> = None;
    let mut closest_wall = 0;
    for (wall_index, wall) in space.walls.iter().enumerate() {
        if let Some(hit) = ray.cast(LineSegment::from_wall(wall)) {
            if closest_hit.is_none() {
                closest_wall = wall_index;
                closest_hit = Some(hit);
            } else {
                closest_hit = closest_hit.map(|closest_hit| {
                    if hit.ray_length < closest_hit.ray_length {
                        closest_wall = wall_index;
                        hit
                    } else {
                        closest_hit
                    }
                })
            }
        }
    }
    closest_hit.map(|ray_hit| WallHit {
        ray: ray_hit,
        wall: WallIndex(closest_wall),
    })
}

#[derive(Debug, Clone, Copy)]
struct SpeakerTrace {
    ray: Ray,
    distance_to_speaker: f32,
}

/// Traces from the given start point to the speaker, and returns a trace if the speaker can be
/// reached. Otherwise returns None.
fn trace_to_speaker(start: Vec2, space: &Space, speaker: &Speaker) -> Option<SpeakerTrace> {
    let direction_unnormalized = speaker.position - start;
    let distance = direction_unnormalized.length();
    let direction = direction_unnormalized / distance;

    let ray = Ray { start, direction };
    let trace = SpeakerTrace {
        ray,
        distance_to_speaker: distance,
    };

    if let Some(hit) = trace_to_walls(ray, space) {
        (hit.ray.ray_length >= distance).then_some(trace)
    } else {
        Some(trace)
    }
}
