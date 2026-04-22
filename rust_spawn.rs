use std::thread;
use std::time::Duration;
use std::vec;

struct VecWrapper<T> {
    v: Vec<T>,
}

impl<T> Drop for VecWrapper<T> {
    fn drop(&mut self) {
        println!("VecWrapper::drop() called.");
    }
}

fn main() {
    let vec_for_thread: VecWrapper<i32> = VecWrapper {
        v: vec![1, 2, 3, 4],
    };

    println!("main thread: @vec_for_thread => {:?}", vec_for_thread.v);
    println!("main thread: spawn new thread.");
    let thread_handle = thread::spawn(
        move || {
            println!("new thread: thread start up.");
            println!("new thread: @vec_for_thread => {:?}", vec_for_thread.v);
            println!("new thread: exit...");
        }
    );

    // let _:() = thread_handle; // determine what the type is.
    
    println!("main thread: sleep 1s.");
    thread::sleep(Duration::from_millis(100));
    println!("main thread: wait for new thread finish.");
    thread_handle.join().unwrap();
    println!("main thread: new thread have joined.");
    println!("main thread: exit...");
}
