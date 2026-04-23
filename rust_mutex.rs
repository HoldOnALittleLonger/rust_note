use std::thread;
use std::sync::Mutex;
use std::sync::Arc;

fn main() {
    let arc: Arc<Mutex<i32>> =
        Arc::new(Mutex::new(20));

    let arc_for_thread = Arc::clone(&arc);
    let thread_handle = thread::spawn(
        move || {
            let mut v = arc_for_thread.lock().unwrap();
            println!("@v is {}", *v);
            *v += 1;
            println!("thread down.");
        }
    );

    println!("in main,@v is {}", *arc.lock().unwrap());
    thread_handle.join().unwrap();
    println!("main finished.");
}
