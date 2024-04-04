use std::rc::Rc;

// #[derive(Debug)]
// struct Foo {
//     v: Box<i32>,
// }

// #[derive(Debug)]
// struct Bar {
//     v: Rc<i32>,
// }

fn main() {
    let mut v1 = Box::new(123);
    // println!("{:p}", v1);
    // println!("{:p}", v1.clone());
    println!("{:?}", v1);
    *v1 = 333;
    println!("{:?}", v1);


    let v2= Rc::new(222);
    println!("{:?}", v2);
    // println!("{:p}", v2);
    // println!("{:p}", v2.clone());
    // println!("{:p}", v2.clone());

    // let f1 = Foo { v: v1 };
    // let f2 = Foo { v: a };

    // let v2 = Rc::new(222);
    // let b1 = Bar { v: v2.clone() };
    // let b2 = Bar { v: v2 };
    // println!("{:?}", b1);
    // println!("{:?}", b2);
}
