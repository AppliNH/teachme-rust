use std::sync::mpsc::channel;
use std::sync::Mutex;

pub struct ThreadPool {
    // Private attribute
    _handles: Vec<std::thread::JoinHandle<()>> 
}

impl ThreadPool {
    pub fn new(nb_threads: u8) -> Self {

        // Channel allow to setup communication between threads.
        // The box is a heap allocator pointer, that will point to a function => the work

        // When you call a function in Rust, it jumps to the function and executes it.
        // When you define you function generically (fn foot<T> (item:T))
        // and that you call it with a u8 or i32 somewhere in your code
        // it will create, in the binary, an implementation of your function for each type, u8 or i32.

        let (sender, receiver) = channel::<Box<dyn Fn()>>();

        // Make the receiver safe by using a Mutex.
        // A mutex basically allow the thread to access the receiver only if the thread holds the receiver.
        // Very good explaination here : https://stackoverflow.com/questions/34524/what-is-a-mutex
        let receiver = Mutex::new(receiver);

        // Creating the threads recursively
        // same as "for _ in ..."  except that it involves a map there
        // and puts it in a variable as a vector of threads.
        let _handles = (0..nb_threads).map(|_| {

            std::thread::spawn(|| loop {
                    // check for work (receive message)
                    // Careful ! This way, you don't listen to the errors of the receiver
                    // Because of the Mutex, there MIGHT be errors do the fact the thread can't hold the receiver
                    // because it might be busy with another thread or so.
                    // So this will eventually make everything crash
                    let work = receiver.lock().unwrap().recv().unwrap();
                    work(); // Call the function
                }
            ) // No semi-colon here ! We create and RETURN a thread recursively.
                
        }).collect();


        Self {
            _handles
        }
    }

    // Execute implements a clojure fn thread (work)
    // Basically, "work" is a callback function
    pub fn execute<T: Fn()>(&self, work: T) {}
}




mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let pool: ThreadPool = ThreadPool::new(5);
        pool.execute(|| println!("Hello from this thread :)"));
        pool.execute(|| println!("Hello from this other thread !"));

    }
}