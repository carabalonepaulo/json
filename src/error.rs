#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("max depth exceeded")]
    MaxDepthExceeded,
    #[error("invalid json key")]
    InvalidKey,
    #[error("unsupported type for json")]
    UnsupportedValue,
    #[error(transparent)]
    Parse(#[from] simd_json::Error),
    #[error(transparent)]
    Ljr(#[from] ljr::error::Error),
    #[error("int out of range")]
    IntOutOfRange,
    #[error("non-finite float value ({0}) is not valid json")]
    NonFiniteNumber(f64),
}
