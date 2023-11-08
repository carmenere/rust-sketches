use socket_api::errors::MyError;
use socket_api::thread_pool;
use std::error::Error;

use std::sync::Mutex;
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::Arc;

pub struct ThreadPool {
    threads: Vec<JoinHandle<()>>,
    sender: Sender<Job>
}

type Job = Box<dyn FnOnce() -> Result<(), Box<dyn Error>> + Send + 'static>;

impl ThreadPool {
    pub fn new(theads: u8) -> Result<Self, MyError> {
        if theads == 0 {
            MyError::new("Number of threads must be greter then 0.");
        }

        // let t: (Sender<Job>, Receiver<Job>) = mpsc::channel();
        let (tx, rx) = mpsc::channel();
        let rx: Arc<Mutex<mpsc::Receiver<Job>>> = Arc::new(Mutex::new(rx));

        Ok(Self {
            threads: (1..=theads).map(|_| {let rx = rx.clone(); thread::spawn(move || loop {
                let job = rx
                    .lock()
                    .unwrap()
                    .recv()
                    .unwrap();
                let r  = job();
            })}).collect(),
            sender: tx
        })
    }
}

impl thread_pool::ThreadPool for ThreadPool {
    fn execute<F>(&self, f: F) -> Result<(), Box<dyn Error>>
    where 
        F: FnOnce() -> Result<(), Box<dyn Error>> + Send + 'static
    {
        match self.sender.send(Box::new(f)) {
            Ok(r) => Ok(r),
            Err(e) => Err(Box::new(MyError::new(&e.to_string()))),
        }
    }
}