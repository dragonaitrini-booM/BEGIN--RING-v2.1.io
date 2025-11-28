use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum DragonError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Phase coherence violation: {0}° (tolerance: {1}°)")]
    PhaseCoherenceViolation(f64, f64),

    #[error("Cryptographic verification failed")]
    CryptoError,

    #[error("Ring submission failed")]
    RingSubmissionError,

    #[error("Protocol error: {0}")]
    Protocol(String),
}

pub type Result<T> = std::result::Result<T, DragonError>;
