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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_messages() {
        assert_eq!(format!("{}", ExpectationError::Null), "Expected null");
        assert_eq!(format!("{}", ExpectationError::Array), "Expected array");
        assert_eq!(format!("{}", ExpectationError::ArrayOfLength(5)), "Expected array of length 5");
        assert_eq!(format!("{}", ExpectationError::Number), "Expected number");
        assert_eq!(format!("{}", ExpectationError::String), "Expected string");
        assert_eq!(format!("{}", ExpectationError::Boolean), "Expected boolean");
        assert_eq!(format!("{}", ExpectationError::Object), "Expected object");
    }
}