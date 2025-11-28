use thiserror::Error;

#[derive(Error, Debug)]
pub enum DragonError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Phase drift exceeded tolerance: {0}°")]
    PhaseDriftExceeded(f64),

    #[error("Chain integrity violated")]
    ChainIntegrityViolation,

    #[error("Atomic write failed: {0}")]
    AtomicWriteFailed(String),

    #[error("io_uring queue full")]
    QueueFull,

    #[error("Invalid record: {0}")]
    InvalidRecord(String),
}

pub type Result<T> = std::result::Result<T, DragonError>;
