#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

struct Closure<F> {
    data: (u8, u16),
    func: F,
}

impl<F> Closure<F>
where
    F: Fn(&(u8, u16)) -> &u8,
{
    fn call(&self) -> &u8 {
        (self.func)(&self.data)
    }
}

fn do_it(data: &(u8, u16)) -> &u8 {
    &data.0
}

fn main() {
    let c = Closure {
        data: (1, 3),
        func: do_it,
    };
    println!("ans = {}", c.call());
}
