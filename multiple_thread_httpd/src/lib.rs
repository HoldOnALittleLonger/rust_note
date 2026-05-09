use std::{
    vec,
    thread,
    sync::{Arc, Mutex, mpsc},
};

/*
 * Job - type alias to Smart Pointer of trait object,
 *       the trait object is a callable object and
 *       can be sent between multi-threads,lifetime
 *       is entirely.
 *       actually,a job is a callable object which will
 *       deal with something.
 */
type Job = Box<dyn FnOnce() + Send + 'static>;

/*
 * Worker - worker descriptor used to represents work thread
 * @id:     identifier
 * @local_thread:
 *          thead handle for this worker
 * # this type is private to this crate,used by ThreadPool type.
 */
struct Worker {
    id: usize,
    local_thread: thread::JoinHandle<()>,
}

impl Worker {
    /// Constructor for Worker type.
    /// @id: identifier of this worker.
    /// @arc_rx:
    ///      Atomic Reference Count type wrapped a Mutex object,
    ///      in which a mpsc::Receiver is contained.
    ///      this parameter is cloned from other context,thus it is
    ///      safe to be moved into new thread.
    fn new(id: usize, arc_rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        println!("start worker {id}");
        Self {
            id,
            local_thread: thread::spawn(
                move || {
                    loop {
                        let msg = arc_rx.lock().expect("should got mutex lock.").recv();
                        match msg {
                            Ok(job) => {
                                println!("worker {id} got a job.");
                                job();
                            },
                            _ => {
                                println!("worker {id} getting job failed,quit.");
                                break;
                            },
                        }
/*
                        let job = arc_rx.lock().expect("should got mutex lock.")
                            .recv().unwrap();
                        /* println!() always takes reference than ownership. */
                        println!("worker {id} got a job.");
                        job();
*/
                    }

                    /*
                     *
                     *  MutexGuard will release this mutex at the timepoint
                     *  after once iteration ends but before next iteration
                     *  starts,thus the behavior would not be expected by us.
                     *
                    while let Ok(job) = 
                        arc_rx.lock().expect("should got mutex lock.").recv() {

                        println!("worker {id} got a job.");
                        job();

                    }
                    */

                    }
                ),
            }
    }

    fn id(&self) -> usize {
        self.id
    }

/*
 *  We can not move out the inner from an object through a mutable reference
 *  to that object.
 *  If we need do this,should pass @self(pass ownership) instead pass a reference.
 *  But this will let the object referred in previous context becomes invalid,
 *  if we no longer need this object,we can follow this way.
 *  We can invoke std::mem::take() on the field's mutable reference to take its
 *  ownership,this requires the type of this field implemented Default trait.
 *  Method std::mem::replace() can be used swap the field's value,and returns
 *  the previous value of this field.
 *  If we wrapped the field in Option<>,we can invoke take() method of Option<>
 *  to take the ownership of the Some(v).
 *
    fn thread_handle(&mut self) -> thread::JoinHandle<()> {
        std::mem::take(&mut self.local_thread)
        // std::mem::replace(&mut self.local_thread, @new_value)
    }
*/
}

/*
 * ThreadPoolError - enumeration type to enumerates error status of
 *                   ThreadPool object.
 */
#[derive(Debug)]
pub enum ThreadPoolError {
    TP_ERROR_BAD_AVAILABLE,
}

/*
 * ThreadPool - thread pool to holds workers and a communication
 *              channel to workers
 * @workers:    all workers
 * @request_sender:
 *              mpsc::Sender<> type object used to send Job to
 *              worker
 */
pub struct ThreadPool {
    workers: Vec<Worker>,
    request_sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {

    /// Constructor of ThreadPool type.
    /// @available: specify the maximum number of workers in the pool,
    ///             can not be _zero_.
    pub fn new(available: usize) -> Result<Self, ThreadPoolError> {
        if available == 0 {
            Err(ThreadPoolError::TP_ERROR_BAD_AVAILABLE)
        } else {
            let mut workers_vec: Vec<Worker>
                = Vec::with_capacity(available);

            let (tx, rx) = mpsc::channel();

            /* move @rx */
            let arc_rx = Arc::new(Mutex::new(rx));

            /* initialize workers */
            for id in 0..available {
                workers_vec.push(Worker::new(id, Arc::clone(&arc_rx)));
            }

            Ok(
                Self {
                    workers: workers_vec,
                    /* move @tx */
                    request_sender: Some(tx),
                }
            )
        }
    }

    /// Member method execute() used to send Job to worker.
    /// @f: callable object and is not re-entrant,can be
    ///     sent between multi-threads.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let new_job: Job = Box::new(f);
        self.request_sender.as_ref().unwrap().send(new_job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        std::mem::drop(self.request_sender.take());
        for t in self.workers.drain(..) {
            println!("worker {} shutting down.", t.id());
            t.local_thread.join().unwrap();
        }
    }
}
