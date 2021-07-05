use std::fmt;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    NotConvertError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        Self::IOError(_) => write!(f, "Error occured because of IO"),
        Self::NotConvertError => write!(f, "This instance isn't converted yet.")
      }
    }
  }

impl std::error::Error for Error {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    match self {
      Self::IOError(ref error) => Some(error),
      Self::NotConvertError => None,
    }
  }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
      Self::IOError(error)
    }
}