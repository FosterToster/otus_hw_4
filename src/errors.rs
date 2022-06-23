use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum StorageError {
    StorageError,
}

impl Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Storage error")
    }
}

impl Error for StorageError {}

#[derive(Debug)]
pub enum SmartHomeError {
    NotFound(String),
    NotUnique(String),
    NotEmpty(String),
    StorageError(String),
}

impl Display for SmartHomeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(text) => {
                write!(f, "Item is not found: {}", text)
            }
            Self::NotUnique(text) => {
                write!(f, "Item is not uniqe: {}", text)
            }
            Self::StorageError(text) => {
                write!(f, "Storage returned an error: {}", text)
            }
            Self::NotEmpty(text) => {
                write!(f, "Storage returned an error: {}", text)
            }
        }
    }
}

impl Error for SmartHomeError {}

impl From<StorageError> for SmartHomeError {
    fn from(_: StorageError) -> Self {
        Self::StorageError("Something went wrong".into())
    }
}
