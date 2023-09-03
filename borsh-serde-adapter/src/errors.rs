use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExpectationError {
    #[error("Expected null")]
    Null,

    #[error("Expected array")]
    Array,

    #[error("Expected array of length {0}")]
    ArrayOfLength(u32),

    #[error("Expected number")]
    Number,

    #[error("Expected string")]
    String,

    #[error("Expected boolean")]
    Boolean,

    #[error("Expected object")]
    Object
}