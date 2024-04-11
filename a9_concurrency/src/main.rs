use std::{sync::mpsc::channel, thread};

fn main() {
    let (tx, rx) = channel();
    thread::spawn(move || {
        tx.send(1).unwrap();
    });

    // rx.recv().unwrap();
    for r in rx {
        println!("rx = {:?}", r);
    }
}
