#![allow(unused_variables)]

fn main() {
    let nums = vec![1, 2, 3];

    {
        let ticket = || nums;
    }

    // let _alp = || {
    //     nums;
    //     vec!['a', 'b']
    // };

    println!("{:?}", nums);
}
