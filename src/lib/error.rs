use std::fmt;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        Self::IOError(_) => write!(f, "Error occured because of IO"),
      }
    }
  }

impl std::error:Error for Error {
  fn source(&self) -> Option<&dyn Error + 'static> {
    match self {
      Self::IOError(ref error) => error
    }
  }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
      Self::IOError(error)
    }
}