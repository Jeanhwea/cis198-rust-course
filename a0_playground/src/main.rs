#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

fn longest<'a>(s: &'a str, t: &'a str) -> &'a str {
    if s.len() > t.len() {
        s
    } else {
        t
    }
}

fn main() {
    let s1 = String::from("abcd");
    let s2 = "xyz";

    let s3 = longest(s1.as_str(), s2);
    println!("s3 = {:?}", s3);
}
