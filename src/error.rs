use std::{error::Error, fmt::Display};

use crate::config::ConversionError;

#[derive(Debug)]
pub enum EditorError {
    Conversion(ConversionError),
}

impl Display for EditorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Conversion(err) => write!(f, "error converting configuration: {}", err),
        }
    }
}

impl Error for EditorError {}

pub type EditorResult<A> = Result<A, EditorError>;

impl From<ConversionError> for EditorError {
    fn from(err: ConversionError) -> Self {
        Self::Conversion(err)
    }
}
