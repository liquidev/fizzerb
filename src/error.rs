use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Pixels error: {0}")]
    Pixels(#[from] pixels::Error),

    #[error("Cairo error: {0}")]
    Cairo(#[from] cairo::Error),
    #[error("Cairo borrow error: {0}")]
    CairoBorrow(#[from] cairo::BorrowError),
}
