use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

//using job to old alias of the data type execute recieve
type Job = Box<dyn FnOnce() + Send + 'static>;

// define a thread pool to hold a limited number of threads
pub struct ThreadPool {
    workers: Vec<Workers>,
    sender: mpsc::Sender<Message>,
}

//define workers to have unive is and a worker of joinhandle type
struct Workers {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
//impl new worker
impl Workers {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Workers {
        //keep worker alive while waiting for next job to be recieved
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });
        Workers {
            id,
            thread: Some(thread),
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        //check if the uzixe is greater than zero
        assert!(size > 0);

        //define the vector capacity of workers
        let mut worker_threads = Vec::with_capacity(size);

        //define sender and reciever from channel
        let (sender, receiver) = mpsc::channel();

        //using arc mutex to make sure multiple workers own a reciever and each worker gets only a job at a time
        let receiver = Arc::new(Mutex::new(receiver));

        //create multiple workers basded on the number of usize
        for i in 0..size {
            worker_threads.push(Workers::new(i, Arc::clone(&receiver)));
        }

        //create new struct with thread holding the worker thread vector
        ThreadPool {
            workers: worker_threads,
            sender: sender,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

//run when going out of scope
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            //when pool is dropped drop would make sure they finish their work
            //call the take method to move the worker out of thread
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
