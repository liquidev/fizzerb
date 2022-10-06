//! Impulse response rendering procedure.

use std::{path::Path, sync::Arc};

use druid::{Data, Lens};
use fizzerb_impulse::{Compressor, ImpulseRenderer};
use fizzerb_model::{MicrophoneIndex, SpeakerIndex};
use fizzerb_tracer::{Tracer, TracerConfig, SPEED_OF_SOUND_IN_AIR};
use hound::{SampleFormat, WavSpec, WavWriter};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::{debug, debug_span, error, info, info_span};

use crate::{error::Error, widgets::data::EditableSpace};

#[derive(Debug, Clone, Data, Lens, Deserialize, Serialize)]
pub struct RenderSettings {
    pub max_bounces: usize,
    pub samples: usize,

    pub speed_of_sound: f32,

    pub compressor_gain: f32,
    pub compressor_threshold: f32,
    pub compressor_release: f32,

    pub sample_rate: u32,
    pub output_path: String,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            max_bounces: 512,
            samples: 256,

            speed_of_sound: SPEED_OF_SOUND_IN_AIR,

            compressor_gain: 1.0,
            compressor_threshold: 0.8,
            compressor_release: 2.0,

            sample_rate: 48000,
            output_path: "impulse_response_#.wav".into(),
        }
    }
}

pub fn render(editable_space: Arc<EditableSpace>, settings: &RenderSettings) {
    let _span = info_span!("render").entered();
    info!(?settings, "use settings");

    debug!("generating renderable model for space");
    let model = editable_space.to_model();
    debug!(
        walls = model.walls.len(),
        microphones = model.microphones.len(),
        speakers = model.speakers.len(),
        "model stats",
    );

    // TEMPORARY: I'm too lazy to mix speakers down at the moment. Excuse my idiocy.
    let speaker = if !model.speakers.is_empty() {
        SpeakerIndex(0)
    } else {
        return;
    };
    let tracer_config = TracerConfig {
        speed_of_sound: settings.speed_of_sound,
        max_bounces: settings.max_bounces,
        record_rays: false,
    };
    for (index, _) in model.microphones.iter().enumerate() {
        let microphone = MicrophoneIndex(index);
        let _span = debug_span!("microphone", ?index).entered();

        debug!("gathering recordings");
        let recordings: Vec<_> = (0..1024)
            .into_par_iter()
            .map(|_| {
                let tracer = Tracer::new(&model, &tracer_config);
                let angle = fastrand::f32() * 2.0 * std::f32::consts::PI;
                let start_ray = glam::Vec2::from_angle(angle);
                tracer.perform_trace(microphone, speaker, start_ray)
            })
            .collect();
        debug!(total = recordings.len(), "recordings gathered");

        debug!("mixing recordings into final impulse");
        let mut impulse_renderer = ImpulseRenderer::new(settings.sample_rate as f32);
        for recording in recordings {
            impulse_renderer.add_responses(&recording.responses);
        }

        debug!("rendering the impulse");
        let impulse_response = impulse_renderer.render(
            settings.compressor_gain,
            Compressor {
                sample_rate: settings.sample_rate as f32,
                threshold: settings.compressor_threshold,
                release: settings.compressor_release,
            },
        );

        save_wav(settings, &impulse_response, index);
    }
}

fn save_wav(settings: &RenderSettings, impulse_response: &[f32], microphone_index: usize) {
    fn inner(
        settings: &RenderSettings,
        impulse_response: &[f32],
        microphone_index: usize,
    ) -> Result<(), Error> {
        let output_path = settings
            .output_path
            .replace('#', &microphone_index.to_string());
        let output_path = Path::new(&output_path);
        debug!(?output_path, "writing wav");

        let spec = WavSpec {
            channels: 1,
            sample_rate: settings.sample_rate as u32,
            bits_per_sample: 32,
            sample_format: SampleFormat::Float,
        };
        let mut writer = WavWriter::create(output_path, spec)?;
        for &sample in impulse_response {
            writer.write_sample(sample)?;
        }
        writer.finalize()?;
        Ok(())
    }

    if let Err(err) = inner(settings, impulse_response, microphone_index) {
        error!(error = %err, "saving wav")
    }
}
