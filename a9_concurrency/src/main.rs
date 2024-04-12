use std::{sync::mpsc::channel, thread};

fn main() {
    let (tx, rx) = channel();

    // sender
    thread::spawn(move || {
        for _ in 0..3 {
            tx.send("xxx").unwrap();
        }

        // tx.send(1).unwrap();
    });

    // reciever
    for r in rx {
        println!("rx = {:?}", r);
    }
}
