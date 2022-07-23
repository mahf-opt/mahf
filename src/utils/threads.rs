//! Helpers for multi threading.

use std::{sync::mpsc, thread};

pub use num_cpus;

type Task = Box<dyn FnOnce() + Send>;

/// A synchronous (blocking) thread pool.
///
/// This allows executing a specific number of tasks in parallel,
/// but [`SyncThreadPool::enqueue`] will block when all available workers are busy.
pub struct SyncThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
    pool: mpsc::Receiver<mpsc::Sender<Task>>,
}

impl Default for SyncThreadPool {
    /// This is equivalent to `SyncThreadPool::new(num_cpus::get())`.
    fn default() -> Self {
        Self::new(num_cpus::get())
    }
}

impl SyncThreadPool {
    /// Creates a sync thread pool with a custom number of workers.
    ///
    /// Use [`SyncThreadPool::default`] to create a thread poll with
    /// one worker per processor thread.
    pub fn new(workers: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let mut pool = SyncThreadPool {
            threads: Vec::with_capacity(workers),
            pool: receiver,
        };

        for _ in 0..workers {
            let pool_sender = sender.clone();
            pool.threads.push(thread::spawn(move || {
                let (tx, rx) = mpsc::channel();
                if pool_sender.send(tx.clone()).is_err() {
                    return;
                }
                while let Ok(task) = rx.recv() {
                    task();
                    if pool_sender.send(tx.clone()).is_err() {
                        return;
                    };
                }
            }));
        }

        pool
    }

    /// Enqueues a new task.
    ///
    /// This will be non-blocking until all workers are occupied.
    /// Once that is the case, it will block until a worker becomes
    /// available again.
    pub fn enqueue(&mut self, task: impl FnOnce() + Send + 'static) {
        self.pool.recv().unwrap().send(Box::new(task)).unwrap();
    }
}
