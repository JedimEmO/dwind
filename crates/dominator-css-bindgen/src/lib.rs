use std::error::Error;
use std::io;
use cssparser::{BasicParseError, ParseError};
use thiserror::Error;

pub mod css;

#[derive(Error, Debug)]
pub enum DCssError {
    #[error("failed reading css file")]
    File(#[from] io::Error),
    #[error("failed parsing css file")]
    CssParse(String),
}

impl<'a> From<BasicParseError<'a>> for DCssError {
    fn from(value: BasicParseError<'a>) -> Self {
        Self::CssParse(format!("{value:?}"))
    }
}


impl<'a, E> From<ParseError<'a, E>> for DCssError {
    fn from(value: ParseError<'a, E>) -> Self {
        value.basic().into()
    }
}

pub type DCssResult<T> = Result<T, DCssError>;