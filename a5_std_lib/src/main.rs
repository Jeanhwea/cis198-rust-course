use std::rc::Rc;

#[allow(unused_variables)]
#[allow(dead_code)]

fn main() {
    // let mut v1 = Box::new(123);
    // *v1 = 222;
    // println!("v1 = {}", v1);

    let mut v2 =  Rc::new(123);
    // *v2 = 222;
    let r1 = Rc::get_mut(&mut v2);
    println!("r1 = {:?}", r1);
    // *r1.unwrap() = 444;
    // println!("v2 = {}", v2);
}
