use socket_api::errors::MyError;
use socket_api::thread_pool;
use std::error::Error;

use std::sync::Mutex;
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::Arc;

pub struct ThreadPool {
    threads: Vec<JoinHandle<()>>,
    sender: Sender<Task>
}

enum Task {
    Job(Job),
    Terminate
}

type Job = Box<dyn FnOnce() -> Result<(), Box<dyn Error>> + Send + 'static>;

impl ThreadPool {
    pub fn drop(self) {
        self.threads.iter().for_each(|_| self.sender.send(Task::Terminate).unwrap());

        for t in self.threads {
            t.join().unwrap()
        }
    }

    pub fn new(theads: u8) -> Result<Self, MyError> {
        if theads == 0 {
            MyError::new("Number of threads must be greter then 0.");
        }

        // let t: (Sender<Job>, Receiver<Job>) = mpsc::channel();
        let (tx, rx) = mpsc::channel::<Task>();
        let rx = Arc::new(Mutex::new(rx));

        Ok(Self {
            threads: (1..=theads).map(|i| {let rx = rx.clone(); thread::spawn(move || {
            println!("Create thread number {i}.");
            loop {
                let job = rx
                    .lock()
                    .unwrap()
                    .recv()
                    .unwrap();
                match job {
                    Task::Job(job) => {
                        job();
                    },
                    Task::Terminate => {
                        break;
                    },
                }
            };
            println!("Terminating...");
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
        match self.sender.send(Task::Job(Box::new(f))) {
            Ok(r) => Ok(r),
            Err(e) => Err(Box::new(MyError::new(&e.to_string()))),
        }
    }
}

// impl Drop for ThreadPool {
//     fn drop(&mut self) {
//         self.threads.iter().for_each(|_| self.sender.send(Task::Terminate).unwrap());

//         for t in &self.threads {
//             t.join().unwrap()
//         }
//     }
// }