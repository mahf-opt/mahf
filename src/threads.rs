use std::{sync::mpsc, thread};

pub use num_cpus;

type Task = Box<dyn FnOnce() + Send>;

pub struct SyncThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
    pool: mpsc::Receiver<mpsc::Sender<Task>>,
}

impl Default for SyncThreadPool {
    fn default() -> Self {
        Self::new(num_cpus::get())
    }
}

impl SyncThreadPool {
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

    pub fn enqueue(&mut self, task: impl FnOnce() + Send + 'static) {
        self.pool.recv().unwrap().send(Box::new(task)).unwrap();
    }
}
