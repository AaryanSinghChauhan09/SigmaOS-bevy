//! Error types

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Initialization error: {0}")]
    Init(String),
    
    #[error("Runtime error: {0}")]
    Runtime(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Operation failed: {0}")]
    OperationFailed(String),
}
