#[derive(Debug)]
struct StrToken<'a> {
    raw: &'a str,
}

impl<'a> StrToken<'a> {
    pub fn new(raw: &'a str) -> StrToken<'a> {
        StrToken { raw: raw }
    }
}

fn main() {
    let secret = "Hello, world!".to_string();
    let token = StrToken::new(&secret[..]);
    println!("token = {:?}", token);
}
