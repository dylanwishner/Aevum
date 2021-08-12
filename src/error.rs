use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct AevumError;
impl Display for AevumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occured")
    }
}

impl Error for AevumError {}
