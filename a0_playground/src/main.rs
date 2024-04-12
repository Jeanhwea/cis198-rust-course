#![allow(unused_variables)]

use std::sync::RwLock;

fn main() {
    let lock = RwLock::new(0);

    // let r1 = lock.read().unwrap();
    // let r2 = lock.read().unwrap();
    // println!("read: {:?}", lock.read().unwrap());
    // println!("read: {:?}", lock.read().unwrap());

    // let mut w1 = lock.write().unwrap();
    // *w1 = 1;
    // let mut w2 = lock.write().unwrap();
    // *w2 = 2;
    // println!("write: {:?}", lock.write().unwrap());
    // println!("write: {:?}", lock.write().unwrap());
}
