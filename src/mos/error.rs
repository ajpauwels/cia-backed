use chrono;
use core::fmt::Debug;
use cssparser::{BasicParseErrorKind, ParseErrorKind};
use std::fmt;

#[derive(Debug)]
pub struct TaggedError {
    msg: String,
}

pub fn new(msg: &str) -> TaggedError {
    TaggedError {
        msg: msg.to_string(),
    }
}

impl fmt::Display for TaggedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl<'i, E: Debug> From<cssparser::ParseError<'i, E>> for TaggedError {
    fn from(this: cssparser::ParseError<'i, E>) -> Self {
        match this.kind {
            ParseErrorKind::Basic(bpe) => match bpe {
                BasicParseErrorKind::UnexpectedToken(t) => TaggedError {
                    msg: format!("found unexpected token: {:?}", t),
                },
                _ => TaggedError {
                    msg: format!("some other error"),
                },
            },
            ParseErrorKind::Custom(e) => TaggedError {
                msg: format!("{:?}", e),
            },
        }
    }
}

impl From<reqwest::Error> for TaggedError {
    fn from(this: reqwest::Error) -> Self {
        TaggedError {
            msg: format!("{}", this),
        }
    }
}

impl From<chrono::format::ParseError> for TaggedError {
    fn from(this: chrono::format::ParseError) -> Self {
        TaggedError {
            msg: format!("{}", this),
        }
    }
}

impl From<regex::Error> for TaggedError {
    fn from(this: regex::Error) -> Self {
        TaggedError {
            msg: format!("{}", this),
        }
    }
}

impl std::error::Error for TaggedError {}
