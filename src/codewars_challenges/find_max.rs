pub fn run_find_max() {
    minimum(&[-52, 56, 30, 29, -54, 0, -110]);
    maximum(&[-52, 56, 30, 29, -54, 0, -110]);
}

// longer but easier to read
fn minimum(arr: &[i32]) -> i32 {
    let min = *arr.iter().min().unwrap();
    println!("{}", min);
    return min;
}

// cleanest way of returning the correct value
fn maximum(arr: &[i32]) -> i32 {
    *arr.iter().max().unwrap()
}