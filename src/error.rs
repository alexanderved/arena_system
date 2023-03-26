use thiserror::Error;

use vec_cell::BorrowError;

pub type ArenaResult<T> = Result<T, ArenaError>;

#[derive(Debug, Error)]
pub enum ArenaError {
    #[error("failed to borrow element: {0}")]
    BorrowError(#[from] BorrowError),
    #[error("trying to use invalid Index")]
    InvalidIndex,
}