pub fn run_stray() {
    let vector = vec![0, 0, 0, 0, 1, 0];
    let vecb = vec![3, 2, 2, 2, 2];

    stray(&vector);
    stray(&vecb);
}

pub fn stray(arr: &[u32]) -> u32 {
    // the fold method on iter takes a starting value and a closure that will use the bitwise
    // XOR operator to return a value if it is unique.
    let unique_num = arr.iter().fold(0, |acc, el| acc ^ el);

    println!("{}", unique_num);
    return unique_num;
}
