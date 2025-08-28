#![allow(unused)]

use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, SendError},
    },
    thread,
};

pub trait HelloMacro {
    fn hello_macro();
}

// src/lib.rs
pub fn greet() {
    println!("Hello from the library!");
}
#[cfg(test)]

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod demo;
#[cfg(test)]
mod tests {

    use super::*;
    use demo::struct_1;
    use std::panic;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn bulid_user() {
        let email = String::from("email");
        let name = String::from("name");
        let u = struct_1::build_user(&email, &name, true);

        let u2;
        {
            let email2 = String::from("email");
            {
                let name2 = String::from("name");
                // 返回的u2生命周期和email2以及name2最短的一致
                u2 = struct_1::build_user(&email2, &name2, true);
            }
            // println!("{:?}", u2.email);
        }
        // println!("{:?}", u2.email);
    }
}

#[derive(Debug)]
pub struct ServerError {
    message: String,
}

impl<T> From<SendError<T>> for ServerError {
    fn from(value: SendError<T>) -> Self {
        ServerError {
            message: value.to_string(),
        }
    }
}

// 线程池实现
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in self.workers.drain(..) {
            println!("Shutdown worker {}", worker.id);
            worker.thread.join().unwrap();
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        // 安全共享，Arc允许多个线程同时持有，Mutex保证同时只能有一个Worker
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(Worker::new(i, receiver.clone()));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F) -> Result<(), ServerError>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job)?;
        Ok(())
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // lock
                let job = match receiver.lock() {
                    Ok(receiver) => match receiver.recv() {
                        Ok(job) => job,
                        Err(e) => panic!("receive error: {}", e),
                    },
                    Err(e) => panic!("lock error: {}", e),
                };
                // unlock
                println!("Worker {id} receive job. executing");
                job();
            }
        });
        Worker {
            id: id,
            thread: thread,
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
