// extern crate rand;
// use rand::Rng;

use rand::Rng;

// use a6_misc::english::greetings;
use a6_misc::chinese::greetings;

fn main() {
    greetings::say();

    let mut r = rand::thread_rng();
    for _ in 1..=3 {
        println!("rand = {}", r.gen::<u8>());
    }
}
