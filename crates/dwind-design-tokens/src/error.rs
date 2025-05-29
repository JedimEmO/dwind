use thiserror::Error;

/// Errors that can occur when working with design tokens
#[derive(Debug, Error)]
pub enum TokenError {
    #[error("Token not found: {0}")]
    NotFound(String),

    #[error("Circular reference detected: {0}")]
    CircularReference(String),

    #[error("Invalid expression: {0}")]
    InvalidExpression(String),

    #[error("Type mismatch in token: {0}")]
    TypeMismatch(String),

    #[error("Deserialization error: {0}")]
    Deserialization(#[from] serde_json::Error),

    #[error("Expression parsing error: {0}")]
    ExpressionParsing(String),

    #[error("Invalid token value: {0}")]
    InvalidValue(String),

    #[error("Missing required field: {0}")]
    MissingField(String),
}

/// Result type for token operations
pub type TokenResult<T> = Result<T, TokenError>;