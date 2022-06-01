use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct StorageError {}

impl Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Storage error")
    }
}

impl Error for StorageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct SmartHomeError {
    pub text: String,
}

impl Display for SmartHomeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Error for SmartHomeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
