use std::fmt;

#[derive(Debug)]
pub enum Error {
    
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "error!");
    }
  }

impl std::error:Error for Error {
  fn description() {
    
  }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {

    }
}