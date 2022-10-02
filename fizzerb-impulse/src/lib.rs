//! Renderer for impulse responses.

mod compressor;

use compressor::Compressor;
use fizzerb_model::Response;

pub use compressor::CompressorConfig;

#[derive(Debug, Clone)]
pub struct ImpulseRenderer {
    pub sample_rate: f32,
    sample_period: f32,
    audio_buffer: Vec<f32>,
    responses_in_buffer: usize,
}

impl ImpulseRenderer {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            sample_period: 1.0 / sample_rate,
            audio_buffer: vec![],
            responses_in_buffer: 0,
        }
    }

    fn add_response(&mut self, response: Response) {
        assert!(response.time > 0.0);

        let position = (response.time / self.sample_period) as usize;
        self.audio_buffer[position] += response.loudness;
        self.audio_buffer[position + 1] -= response.loudness;
    }

    /// Adds the given bounce responses into the audio buffer.
    ///
    /// The `responses` buffer is assumed to be sorted from earlie.
    pub fn add_responses(&mut self, responses: &[Response]) {
        if responses.is_empty() {
            return;
        }

        let last_time = responses.last().unwrap().time;
        log::debug!(
            "last_time = {last_time}, sample_rate = {}, sample_period = {}",
            self.sample_rate,
            self.sample_period
        );
        assert!(last_time > 0.0);
        let required_buffer_size = (last_time / self.sample_period).ceil() as usize + 2;
        if self.audio_buffer.len() < required_buffer_size {
            log::debug!("resizing sample buffer to {required_buffer_size}");
            self.audio_buffer.resize(required_buffer_size, 0.0);
        }

        for &response in responses {
            self.add_response(response);
        }

        self.responses_in_buffer += 1;
    }

    /// Renders the audio buffer into a finished sample.
    pub fn render(&self, gain: f32, compressor_config: CompressorConfig) -> Vec<f32> {
        let mut output = self.audio_buffer.clone();
        let truncated_length = (!output.is_empty())
            .then(|| output.iter().rposition(|&x| x > 0.00001))
            .flatten()
            .unwrap_or(0);
        output.resize(truncated_length, 0.0);
        log::debug!("rendering sample with length {truncated_length}");

        let mut input = output.clone();
        for sample in &mut input {
            *sample *= gain;
        }

        let compressor = Compressor::new(compressor_config);
        compressor.run(&input[..truncated_length], &mut output);

        output
    }
}

/// The loudness mapping function.
fn loudness(x: f32) -> f32 {
    2.0 * (1.0 / (1.0 + std::f32::consts::E.powf(-5.0 * x)) - 0.5)
}