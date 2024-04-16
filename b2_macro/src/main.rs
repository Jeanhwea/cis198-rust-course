macro_rules! incr {
    ($x:ident) => {
        $x += 1;
    };
}

fn main() {
    let mut x = 0;

    incr!(x); // x += 1;

    println!("x = {x}");
}
