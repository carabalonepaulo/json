#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cryclic table in json")]
    CyclicValue,
    #[error("invalid json key")]
    InvalidKey,
    #[error("unsupported type for json")]
    UnsupportedValue,
}
