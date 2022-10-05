use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("platform error: {0}")]
    Platform(#[from] druid::PlatformError),

    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("WAV error: {0}")]
    Wav(#[from] hound::Error),
}
