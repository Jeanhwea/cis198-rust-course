fn inc(slice: &mut [i32]) {
    if slice.len() < 10 {
        for p in slice {
            *p += 1;
        }
    } else {
        let mid = slice.len() / 2;
        let (left, right) = slice.split_at_mut(mid);
        rayon::join(|| inc(left), || inc(right));
    }
}

fn main() {
    let mut x = vec![1; 100];
    inc(&mut x);
    println!("x = {:?}", x);
}
