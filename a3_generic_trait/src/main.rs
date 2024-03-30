use std::fmt::{self, Display};

// Trait 类似于 Java 的接口或 Haskell 的 typeclass
trait PrettyPrint {
    fn format(&self) -> String;
}

// 使用标准库中的 Result
// #[derive(Debug)]
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

impl<T, E> PrettyPrint for Result<T, E>
where
    T: PrettyPrint,
    E: PrettyPrint,
{
    fn format(&self) -> String {
        match self {
            Ok(t) => format!("Ok({})", t.format()),
            Err(e) => format!("Err({})", e.format()),
        }
    }
}

trait Equals {
    fn equals(&self, other: &Self) -> bool;
}

impl<T, E> Equals for Result<T, E>
where
    T: Equals,
    E: Equals,
{
    fn equals(&self, other: &Self) -> bool {
        match (self, other) {
            (Ok(t1), Ok(t2)) => t1.equals(&t2),
            (Err(e1), Err(e2)) => e1.equals(&e2),
            _ => false,
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PPP({},{})", self.x, self.y)
    }
}

fn main() {
    let p0 = Point { x: 0, y: 0 };
    println!("{}", p0);
    // let ok1:Result::<String, String> = Result::Ok("1".to_string());
    // let ok2 = Result::Ok::<String, String>("1".to_string());
    // println!("{:?}", ok1);
    // println!("{:?}", ok2);
    // println!("{:?}", ok1.equals(&ok2));
}
