use std::thread::{self, JoinHandle};

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Creates a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if `size` is zero.
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let workers: Vec<_> = (0..size).map(|id| Worker::new(id)).collect();

        Self { workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker {
    id: usize,
    handle: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Self {
        let handle = thread::spawn(|| {});
        Self { id, handle }
    }
}
