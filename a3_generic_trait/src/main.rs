#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Line {
    from: Point,
    to: Point,
}

// Trait 类似于 Java 的接口或 Haskell 的 typeclass
trait PrettyPrint {
    fn format(&self) -> String;
}

// 实现 Trait
impl PrettyPrint for Point {
    fn format(&self) -> String {
        format!("({},{})", self.x, self.y)
    }
}

impl PrettyPrint for Line {
    fn format(&self) -> String {
        format!("[{}->{}]", self.from.format(), self.to.format())
    }
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

fn main() {
    // let p0 = Point { x: 0, y: 0 };
    // let p1 = Point { x: 1, y: 1 };
    // let line = Line { from: p0, to: p1 };
    // println!("{}", line.format());

    // let p1 = Point{x:1, y:0};
    // println!("{}", p0.equals(p1));

    // let e1: Result<String, String> = Result::Ok("xxx".to_string());
    // println!("{:?}", e1);

    // let e2: Result<String, String> = Result::Err("yyy".to_string());
    // println!("{:?}", e2);

    let ok1:Result::<String, String> = Result::Ok("1".to_string());
    let ok2:Result::<String, String> = Result::Ok("1".to_string());
    println!("{:?}", ok1);
    println!("{:?}", ok2);
    // println!("{:?}", ok1.equals(&ok2));
}
