use std::fmt::Display;
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/// A callable to be send to a thread for execution.
type Job = Box<dyn FnOnce() + Send>;

/// A pointer to a reciever object of [Job]s that is shared between workers in a
/// mutually exclusive fashion [Mutex].
type MultiReceiver = Arc<Mutex<Receiver<Job>>>;

/// Workers take ownership of the thread which runs [Job]s.
struct Worker {
    id: usize,
    thread_h: thread::JoinHandle<()>
}

impl Worker {
    /// Creates a new worker with thread and id.
    /// 
    /// # Parameters
    /// in_id - is a numeric id assigned by [Pool]. Only used for human comprehension.
    /// in_rx - is a [MultiReceiver].
    fn new(in_id: usize, in_rx: MultiReceiver) -> Worker{
        let thread_h = thread::spawn(
            move || loop {
                let job = in_rx.lock().unwrap().recv();

                match job {
                    Ok(callable) => {
                        println!("worker {in_id} recieved a job.");
                        callable();
                    },
                    Err(_) => {
                        eprintln!("worker {in_id} could not recieve a job. Shutting down.");
                        break;
                    }
                }
            }
        );

        Worker {
            id: in_id,
            thread_h: thread_h
        }
    }
}

impl Display for Worker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "worker {}", self.id)
    }
}

/// Pool owns [Worker] instances and sends them [Job]s for execution.
/// Pool manages graceful shutdown of workers as seen in drop implementation.
pub struct Pool {    
    tx: Option<mpsc::Sender<Job>>,
    workers: Vec<Worker>
}

impl Pool {
    /// Creates a new pool with [Worker]s.
    /// 
    /// # Parameters
    /// n_workers - is a maximum number of [Worker]s to concurrently process [Job]s.
    /// All workers are allocated in the constructor and are only deallocated in drop.
    /// Value of n_workers must be in range [1, 256). While 0 and less workers make zero
    /// sense as is, the upper bound of 255 was chosen to prevent disasterous allocations
    /// of too many [Worker]s' threads to handle.
    /// 
    /// # Panics
    /// Will panic if n_workers is outside of the range [1, 256).
    /// 
    /// # Example
    /// ```
    /// use rns::worker_pool::Pool;
    /// 
    /// let pool = Pool::new(4);
    /// ```
    pub fn new(n_workers: usize) -> Pool {
        assert!(
            n_workers > 0 && n_workers < 256,
            "n_workers must be in range [1, 256)"
        );

        let (tx , rx) = mpsc::channel();
        let rx: MultiReceiver = Arc::new(Mutex::new(rx));
        let mut workers = Vec::with_capacity(n_workers);

        for i in 0..n_workers {
            workers.push(
                Worker::new(i, rx.clone())
            );
        }

        Pool {
            tx: Some(tx),
            workers: workers
        }
    }

    /// Sends callable to be executed by [Worker]s.
    /// 
    /// Parameters
    /// callable - basically any closure, since it must implemments [FnOnce]
    /// (be any closure) and [Send] (to be shared with [Worker]).
    /// 
    /// Example
    /// ```
    /// use rns::worker_pool::Pool;
    /// 
    /// let pool = Pool::new(4);
    /// 
    /// for i in 0..4 {
    ///     pool.execute(
    ///         move || {
    ///             println!("I have {i}!");
    ///         }
    ///     );
    /// }
    /// ```
    pub fn execute<F>(&self, callable: F) 
    where 
        F: FnOnce() + Send + 'static
    {
        let f_ptr: Job = Box::new(callable);

        // Option will always unwrap as long as pool is not dropped.
        if let Err(_) = self.tx.as_ref().unwrap().send(f_ptr) {
            eprintln!("Channel broken. Shutting down.")
        }
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        // Will break channel and force workers to shutdown.
        drop(self.tx.take());

        for w in self.workers.drain(..) {
            w.thread_h.join().unwrap();
        }
    }
}

#[cfg(test)]
mod tests;
