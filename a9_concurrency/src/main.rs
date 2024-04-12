use std::sync::Once;

fn main() {
    static INIT: Once = Once::new();
    for _ in 0..10 {
        INIT.call_once(|| {
            println!("xxx");
        });
    }
}
