pub fn run_narcissistic() {
    // narcissistic(7);
    narcissistic(371);
    // narcissistic(4887);
}

fn narcissistic(num: u64) -> bool {
    // get the length of the number by converting it to a string
    let length = num.to_string().len();
    // println!("{}", length);

    // create a sum variable
    let mut sum: u64 = 0;

    // loop over each number and multiply it by the power of the length
    for num in num.to_string().chars() {
        // sum += num.to_digit(10).map(|num| num.pow(length as u32)).unwrap();
        sum += (num.to_digit(10).unwrap() as u64).pow(length as u32);
    }
    println!("{}", sum);

    if sum == num {
        return true;
    } else {
        return false;
    }
}