use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DbError(#[from] diesel::result::Error),
    #[error("R2D2 error: {0}")]
    R2d2Error(#[from] r2d2::Error),
    // ... other error types
}
