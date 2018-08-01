pub mod config;
pub mod options;
pub mod error;

use std::error::Error;

pub type GenError = Box<Error>;
pub type GenResult<T> = Result<T, GenError>;
