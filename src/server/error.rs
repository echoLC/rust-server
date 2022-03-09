use std::fmt;
use std::io;

pub enum ErrorKind {
  Route,
  File
}

pub struct ServerError {
  pub code: usize,
  pub kind: ErrorKind,
  pub message: String
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let error_msg = match self.code {
          404 => "Page not found",
          _ => "Sorry, something is wrong! Please Try Again!"
      };
      write!(f, "{}", error_msg)
    }
}

impl fmt::Debug for ServerError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "ServerError {{ code: {}, message: {} }}",
      self.code, self.message
    )
  }
}

impl From<io::Error> for ServerError {
    fn from(error: io::Error) -> Self {
      ServerError { code: 1, message: error.to_string(), kind: ErrorKind::File }
    }
}

