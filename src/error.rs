use std::{self};

#[derive(Debug)]
pub enum RoxError {
    Io(std::io::Error),
    Lexical { line: usize, message: String },
}

impl std::fmt::Display for RoxError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RoxError::Io(err) => write!(f, "I/O Error: {}", err),
            RoxError::Lexical { line, message } => {
                write!(f, "[line {}] Lexical error: {}", line, message)
            }
        }
    }
}

impl From<std::io::Error> for RoxError {
    fn from(e: std::io::Error) -> RoxError {
        RoxError::Io(e)
    }
}
