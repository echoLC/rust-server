use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

use crate::server::worker::{Worker};
use crate::server::message::{Message};
use crate::server::error::{ServerError};

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Message>
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
      assert!(size > 0);

      let (sender, receiver) = mpsc::channel();
      let receiver = Arc::new(Mutex::new(receiver));
      let mut workers = Vec::with_capacity(size);

      for id in 0..size {
        workers.push(Worker::new(id, Arc::clone(&receiver)));
      }

      ThreadPool{ workers, sender }
    }

    pub fn execute<F>(&self, f: F) -> Result<(), ServerError>
      where 
        F: FnOnce() + Send + 'static
        {
          let job = Box::new(f);
          self.sender.send(Message::NewJob(job))?;
          Ok(())
        } 
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
      println!("Sending terminate message to all workers.");

      for _ in &self.workers {
        self.sender.send(Message::Terminate).unwrap();
      }

      for worker in &mut self.workers {
        println!("Shutting down worker {}", worker.id);

        if let Some(thread) = worker.thread.take() {
          thread.join().unwrap();
        }
      }
    }
}