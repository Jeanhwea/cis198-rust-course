fn main() {
    let mut v1 = vec![1, 1, 1, 1];
    for v in &mut v1 {
        // println!("{v}");
        *v = 2
    }
    println!("{:?}", v1);

    for (i, v) in enumerate!(&v1) {
        println!("{i}:{v}");
    }
}
