use std::{io::Error, string::FromUtf8Error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("IO error occurred")]
    IO(#[from] Error),
    #[error("Found a non-boolean value")]
    NonBoolValue,
    #[error("Failed UTF-8 conversion")]
    FromUtf8(#[from] FromUtf8Error),
}
