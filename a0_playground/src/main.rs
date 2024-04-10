use core::time;
use std::thread;

fn main() {
    for i in 0..10 {
        thread::spawn(move || {
            println!("thr #{}", i);
        });
    }
    let dur = time::Duration::from_millis(500);
    thread::sleep(dur);
}
