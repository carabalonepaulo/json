#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cryclic table in json")]
    CyclicValue,
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
}
