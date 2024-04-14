fn main() {
    let b1 = Box::new(333);
    // let b2 = b1;
    let ptr3 = &*b1 as *const i32; // 不像 let b2 = b1; 这里没有消费 b1
    unsafe {
        println!("ptr3 = {:?}", *ptr3);
    }
    println!("b1 = {}", b1);
}
