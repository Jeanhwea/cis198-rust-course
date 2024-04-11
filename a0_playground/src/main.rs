use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    for i in 0..3 {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[i] += 1;
        });
    }
    thread::sleep(Duration::from_millis(500));

    println!("{:?}", data.clone().lock().unwrap());
}
