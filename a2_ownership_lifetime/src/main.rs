fn main() {
    let mut v1 = vec![0, 1, 2];

    // Borrow immutably
    // for e in v1.iter() {
    for e in &v1 {
        // Can also write `for e in v1.iter()`
        println!("I'm borrowing {}.", e);
    }

    // Borrow mutably
    // for e in v1.iter_mut() {
    for e in &mut v1 {
        *e = *e + 1;
        println!("I'm mutably borrowing {}.", e);
    }

    // Take ownership of the whole vector
    // for e in v1.into_iter() {
    for e in v1 {
        println!("I now own {}! AHAHAHAHA!", e);
    }

    // println!("{:?}", v1);
}
