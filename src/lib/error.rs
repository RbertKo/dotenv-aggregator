use std::fmt;

#[derive(Debug)]
pub enum Error {
    IOError
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        Self::IOError => write!(f, "Error occured because of IO"),
      }
    }
  }

impl std::error:Error for Error {
  fn source(&self) -> Option<&dyn Error + 'static> {

  }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
      Self::IOError
    }
}