#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

use std::mem::transmute;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn foo() -> i32 {
    123
}

fn main() {
    let pointer = foo as *const ();
    let f1 = unsafe {
        std::mem::transmute::<*const (), fn() -> i32>(pointer)
    };
    println!("ans = {:?}", f1());
}
