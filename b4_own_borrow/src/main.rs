#[derive(Debug)]
struct StrToken<'a> {
    raw: &'a str,
}

impl<'a> StrToken<'a> {
    pub fn new(raw: &'a str) -> StrToken<'a> {
        StrToken { raw: raw }
    }
}

#[derive(Debug)]
struct StringToken {
    raw: String,
}

impl StringToken {
    pub fn new(raw: String) -> StringToken {
        StringToken { raw: raw }
    }
}

struct Token {
    raw: String,
}

impl Token {
    pub fn new<S: Into<String>>(raw: S) -> Token {
        Token { raw: raw.into() }
    }
}

fn main() {
    let secret = "Hello, world!".to_string();
    {
        let token = StrToken::new(&secret[..]);
        println!("token = {:?}", token);
    }
}
