use std::sync::mpsc;
use std::thread;
use std::string;

fn main() {
    let to_be_sent = String::from("to_be_sent");
    println!("@to_be_sent is {to_be_sent}");
    let (tx, rx) = mpsc::channel();
    let thandle = thread::spawn(
        move || {
            println!("new thread: ownership transferred.");
            println!("new thread: @to_be_sent is {to_be_sent}");
            tx.send(to_be_sent).unwrap();
        }
    );

    let received_msg = rx.recv().unwrap();
    println!("@received_msg is {received_msg}");
    thandle.join().unwrap();
}
