use std::cell::{Cell, RefCell};

#[allow(unused_variables)]
#[allow(dead_code)]
#[derive(Debug)]
struct Foo {
    x: Cell<i32>,
    y: RefCell<u32>,
}

fn main() {
    let v1 = RefCell::new(vec![1]);

    let mut inner = v1.borrow_mut();
    inner.push(2);

    println!("v1 = {:?}", v1.borrow());

    let mut inner2 = v1.borrow();
}
