use std::borrow::Cow;

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

#[derive(Debug)]
struct Token<'a> {
    raw: Cow<'a, str>,
}

impl<'a> Token<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(raw: S) -> Token<'a> {
        Token { raw: raw.into() }
    }
}

fn main() {
    // let secret = "Hello, world!".to_string();
    // {
    //     // let token = StringToken::new(secret);
    //     // let token = StrToken::new(&secret[..]);

    //     // let token = Token::new(&secret[..]);
    //     let token = Token::new(secret);

    //     println!("token = {:?}", token);
    // }
    // println!("secret = {:?}", secret);

    let token1 = Token::new(Cow::Borrowed("abc"));
    let s1 = "zyx".to_string();
    let token2 = Token::new(Cow::Owned(s1));
    println!("token1 = {:?}", token1);
    println!("token2 = {:?}", token2);
}
