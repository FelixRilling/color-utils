use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::option::Option::None;

/// Kinds of errors than may happen during color parsing.
#[derive(Debug)]
pub enum ParsingError<'a> {
    InvalidSyntax(&'a str),

    NumberConversionFailed(Box<dyn Error>),
}

impl Display for ParsingError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParsingError::InvalidSyntax(details) => f.write_str(details),
            ParsingError::NumberConversionFailed(_) => f.write_str("Number conversion failed"),
        }
    }
}

impl Error for ParsingError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParsingError::InvalidSyntax(_) => None,
            ParsingError::NumberConversionFailed(err) => Some(&**err),
        }
    }
}

impl From<std::num::ParseFloatError> for ParsingError<'_> {
    fn from(err: std::num::ParseFloatError) -> Self {
        ParsingError::NumberConversionFailed(Box::new(err))
    }
}

impl From<std::num::ParseIntError> for ParsingError<'_> {
    fn from(err: std::num::ParseIntError) -> Self {
        ParsingError::NumberConversionFailed(Box::new(err))
    }
}

impl From<rug::float::ParseFloatError> for ParsingError<'_> {
    fn from(err: rug::float::ParseFloatError) -> Self {
        ParsingError::NumberConversionFailed(Box::new(err))
    }
}

/// Error for [`TryFrom`] conversions.
#[derive(Debug)]
pub struct TryFromError();

impl Display for TryFromError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("Could not convert value.")
    }
}

impl Error for TryFromError {}
