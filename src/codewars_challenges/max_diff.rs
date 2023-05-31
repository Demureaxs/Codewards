pub fn run_max_diff() {
    // max_diff(&[0, 1, 2, 3, 4, 5, 6]);
    max_diff(&[-0, 1, 2, -3, 4, 5, -6]);
}

fn max_diff(numbers: &[i32]) -> i32 {
    if numbers.len() == 1 {
        return 0;
    }
    let max = numbers.iter().max().unwrap();
    let min = numbers.iter().min().unwrap();
    println!("{}", (max - min).abs());
    return (max - min).abs();
}