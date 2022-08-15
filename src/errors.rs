use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("sth went wrong in storage")]
    StorageError,
}

#[derive(Error, Debug)]
pub enum SmartHomeError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Not unique: {0}")]
    NotUnique(String),

    #[error("Not empty: {0}")]
    NotEmpty(String),

    #[error("Storage error: {0}")]
    StorageError(#[from] StorageError),
}
