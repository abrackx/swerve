use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Sqlx error: {0}")]
    Sqlx(#[from] SqlxError),
}