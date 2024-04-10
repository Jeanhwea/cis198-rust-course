use std::{thread, time::Duration};

fn main() {
    for i in 0..10 {
        thread::spawn(move || {
            println!("aaa i = {}", i);
        });
    }

    thread::sleep(Duration::from_millis(1000));
}
