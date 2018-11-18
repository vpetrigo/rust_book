use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("Hi!");
        tx.send(msg).unwrap();
    });

    let resp = rx.recv().unwrap_or_else(|err| {
        eprintln!("Unable to receive message from another thread: {}", err);
        String::from("Unknown")
    });

    println!("Receive: {}", resp);
}
