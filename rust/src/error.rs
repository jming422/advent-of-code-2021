use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    ParseInt,
    Other(String),
}

#[derive(Debug)]
pub struct MyError {
    kind: ErrorKind,
    source_error: Option<Box<dyn Error + 'static>>,
}

impl MyError {
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            kind,
            source_error: None,
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    // These "wrap" functions reduce duplicate code in the common
    // `.map_err(|err| please_turn_this_into_rb_error(err))` type situations
    pub fn wrap_parse_int<E>(err: E) -> Self
    where
        E: Into<Box<dyn Error + 'static>>,
    {
        Self {
            kind: ErrorKind::ParseInt,
            source_error: Some(err.into()),
        }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source_error.as_ref().map(|b| b.as_ref())
    }
}
