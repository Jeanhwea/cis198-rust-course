use std::borrow::BorrowMut;

#[allow(unused_variables)]
#[allow(dead_code)]

fn main() {
    let mut v1 = Box::new(123);
    println!("v1 = {}", v1);
    *v1 = 333;

    let v2 = v1.borrow_mut();
    *v2 = 444;

    println!("v1 = {}", v1);
}
