#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut p = Point { x: 1, y: 2 };
    let x1 = &mut p.x;
    let y1 = &mut p.y;
    *x1 += 1;
    *y1 += 1;
    println!("p = {:?}", p);
}
