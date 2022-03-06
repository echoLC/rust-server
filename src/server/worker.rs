use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use crate::server::message::{ Message };

pub struct Worker {
  pub id: usize,
  pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new (id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
      let thread = thread::spawn(move || loop {
        let message = receiver.lock().unwrap().recv().unwrap();

        match message {
            Message::NewJob(job) => {
              println!("Worker {} got a job; executing.", id);
              job();
            },
            Message::Terminate => {
              println!("Worker {} was told to terminate.", id);

              break;
            }
        }
      });

      Worker{ id, thread: Some(thread) }
    }
}

