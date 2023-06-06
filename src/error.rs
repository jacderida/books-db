use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error(transparent)]
    DatabaseError(#[from] rusqlite::Error),

    #[error("Failed to parse string to model: {0}")]
    ParseError(String),
}
