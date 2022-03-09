use std::fmt;
use std::io;
use std::sync::mpsc::SendError;
use crate::server::message::{Message};

pub enum ErrorKind {
  Route,
  File,
  Sync
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

impl From<SendError<Message>> for ServerError {
    fn from (error: SendError<Message>) -> Self {
      ServerError {code: 2, message: error.to_string(), kind: ErrorKind::Sync}
    }
}

