use parser::ParseError;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum MarkupError {
    Parse(ParseError),
    Render(String),
}

impl fmt::Display for MarkupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MarkupError::Parse(ref err) => write!(f, "Parse error: {}", err),
            MarkupError::Render(ref err) => write!(f, "Render error: {}", err),
        }
    }
}


impl error::Error for MarkupError {
    fn description(&self) -> &str {
        match *self {
            MarkupError::Parse(ref err) => err.description(),
            MarkupError::Render(ref err) => err,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            MarkupError::Parse(ref err) => Some(err),
            MarkupError::Render(_) => None,
        }
    }
}

impl From<ParseError> for MarkupError {
    fn from(err: ParseError) -> MarkupError {
        MarkupError::Parse(err)
    }
}

impl From<String> for MarkupError {
    fn from(err: String) -> MarkupError {
        MarkupError::Render(err)
    }
}
