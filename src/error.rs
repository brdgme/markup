use std::error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum MarkupError {
    Render(String),
}

impl fmt::Display for MarkupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MarkupError::Render(ref err) => write!(f, "Render error: {}", err),
        }
    }
}


impl error::Error for MarkupError {
    fn description(&self) -> &str {
        match *self {
            MarkupError::Render(ref err) => err,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            MarkupError::Render(_) => None,
        }
    }
}

impl From<String> for MarkupError {
    fn from(err: String) -> MarkupError {
        MarkupError::Render(err)
    }
}
