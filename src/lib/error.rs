use std::fmt;

#[derive(Debug)]
pub enum Error {
    
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      f.debug_struct("")
    }
  }

impl std::error:Error for Error {

}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {

    }
}