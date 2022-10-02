pub struct Compressor {
    pub sample_rate: f32,
    pub threshold: f32,
    pub release: f32,
}

impl Compressor {
    pub fn run(&self, input: &[f32], output: &mut [f32]) {
        assert!(input.len() == output.len());

        let release_per_sample = self.sample_rate * self.release;
        let mut compression = 0.0_f32;

        for (i, &sample) in input.iter().enumerate() {
            let over_threshold = (sample.abs() - self.threshold).max(0.0);
            compression = compression.max(over_threshold);

            let loudness = (1.0 - compression).max(0.0);
            output[i] = sample * loudness;

            compression -= release_per_sample;
            compression = compression.max(0.0);
        }
    }
}
