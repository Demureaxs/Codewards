pub fn run_find_difference() {
    find_difference(&[3, 2, 5], &[1, 4, 4]);
    find_difference(&[9, 7, 2], &[5, 2, 2]);
    find_difference(&[11, 2, 5], &[1, 10, 8]);
    find_difference(&[4, 4, 7], &[3, 9, 3]);
    find_difference(&[15, 20, 25], &[10, 30, 25]);
    find_difference(&[12, 28, 10], &[25, 11, 16]);
    codewars_find_difference(&[12, 28, 10], &[25, 11, 16]);
}

fn find_difference(a: &[i32; 3], b: &[i32; 3]) -> i32 {
    let difference = a.iter().fold(1, |acc, cv| acc * cv) - b.iter().fold(1, |acc, cv| acc * cv);
    println!("{}", difference.abs());
    return difference.abs();
}

fn codewars_find_difference(a: &[i32; 3], b: &[i32; 3]) -> i32 {
    a.iter()
        .fold(1, |acc, cv| acc * cv)
        // abs_diff returns the absolute difference between what it is called on and what is
        // called inside
        .abs_diff(b.iter().fold(1, |acc, cv| acc * cv)) as i32
}
