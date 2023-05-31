pub fn run_get_average() {
    get_average(&[1, 2, 15, 15, 17, 11, 12, 17, 17, 14, 13, 15, 6, 11, 8, 7]);
}

fn get_average(marks: &[i32]) -> i32 {
    let result = marks.iter().sum::<i32>() / marks.len() as i32;
    println!("{}", result);

    return result;
}