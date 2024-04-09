use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        thread::park();
        println!("aaa");
    });
    println!("bbb");

    println!("curr = {:?}", thread::current());

    handle.thread().unpark();

    handle.join().unwrap();
}
