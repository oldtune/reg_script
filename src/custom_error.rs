use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum CustomError {
    IOError,
    BusinessErr(String),
}

impl Error for CustomError {}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
