#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u32 {
    fn draw(&self) -> String {
        format!("u32: {}", self)
    }
}

impl Draw for i32 {
    fn draw(&self) -> String {
        format!("i32: {}", self)
    }
}

fn draw1(x: &dyn Draw) {
    x.draw();
}

fn draw2(x: Box<dyn Draw>) {
    x.draw();
}

fn main() {
    let a = 10i32;
    let b = 11u32;

    draw1(&a);
    draw1(&b);
    draw2(Box::new(a));
    draw2(Box::new(b));
}
