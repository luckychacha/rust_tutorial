use thiserror::Error;

#[derive(Error, Debug)]
pub enum RustTutorialError {
    #[error("writeln! error")]
    Disconnect(#[from] std::fmt::Error),
    // #[error("the data for key `{0}` is not available")]
    // Redaction(String),
    // #[error("invalid header (expected {expected:?}, found {found:?})")]
    // InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}
