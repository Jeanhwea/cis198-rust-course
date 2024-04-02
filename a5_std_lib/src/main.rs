fn foo() -> Result<i32, String> {
    // Result::Ok(1)
    Result::Err("xxx".to_string())
}

fn main() {
    // let opt1: Option<i32> = Some(3);
    // println!("ans = {}", opt1.unwrap_or(999));
    // let opt1: Option<i32> = Some(3);
    // println!("opt1 = {:?}", opt1);
    // let opt2 = opt1.and_then(|x| Some(x + 1));
    // println!("opt2 = {:?}", opt2);

    // let ok1: Result<i32, String> = ;
    // let err1: Result<i32, String> = Result::Err("we failed".to_string());
    match foo() {
        Ok(code) => println!("code = {code}"),
        Err(msg) => println!("msg = {msg}"),
    }

}
