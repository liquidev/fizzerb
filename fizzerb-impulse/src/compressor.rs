use std::fs::File;
use std::io::Write;

/// Compressor options.
pub struct CompressorConfig {
    pub sample_rate: f32,
    pub threshold: f32,
    pub release: f32,
}

pub struct Compressor {
    config: CompressorConfig,
    sample_period: f32,
}

impl Compressor {
    pub fn new(config: CompressorConfig) -> Self {
        let sample_period = 1.0 / config.sample_rate;
        Self {
            config,
            sample_period,
        }
    }

    pub fn run(&self, input: &[f32], output: &mut [f32]) {
        assert!(input.len() == output.len());

        let mut debug_csv = File::create("/tmp/fizzerb_compressor_debug.csv").unwrap();
        writeln!(
            debug_csv,
            "i,sample,compression,over_threshold,loudness,output"
        )
        .unwrap();

        let release_per_sample = self.config.sample_rate * self.config.release;
        let mut compression = 0.0_f32;

        for (i, &sample) in input.iter().enumerate() {
            let over_threshold = (sample.abs() - self.config.threshold).max(0.0);
            compression = compression.max(over_threshold);

            let loudness = (1.0 - compression).max(0.0);
            output[i] = sample * loudness;

            writeln!(
                debug_csv,
                "{i},{sample},{compression},{over_threshold},{loudness},{}",
                output[i]
            )
            .unwrap();

            compression -= release_per_sample;
            compression = compression.max(0.0);
        }
    }
}
