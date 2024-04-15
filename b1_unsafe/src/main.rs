#[link(name = "ffi", kind = "dylib")]
extern "C" {
    fn foo() -> i32;
}

fn main() {
    println!("foo() = {:?}", unsafe { foo() });
}
