use std::{io::Error, num::TryFromIntError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error("IO error occurred")]
    IO(#[from] Error),
    #[error("Error occurred during conversion from integer")]
    TryFromInt(#[from] TryFromIntError),
}
