#[link(name = "ffi", kind = "dylib")]
extern "C" {
    fn foo() -> i32;
    fn fib(n: i32) -> i32;
}

fn main() {
    println!("foo() = {:?}", unsafe { foo() });

    for i in 0..9 {
        unsafe {
            println!("fib({}) = {}", i, fib(i));
        }
    }
}
