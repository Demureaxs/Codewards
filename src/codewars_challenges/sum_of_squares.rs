pub fn run_sum_of_squares() {
    sum_of_squares(15);
    // sum_of_squares(16);
    // sum_of_squares(18);
    // sum_of_squares(17);
}

fn sum_of_squares(n: u64) -> u64 {
    if is_square(n) {
        return 1;
    }
    // set a mutable variable n
    let mut n = n;

    // while n can be divided by 4
    while n % 4 == 0 {
        // divide by 4
        n /= 4;
    }

    println!("{}", n);

    // if n remainder 8 is equal to 7 return 4
    if n % 8 == 7 {
        return 4;
    }

    // otherwise loop from 0 to the square root of n as an integer
    for num in 0..=(n as f64).sqrt() as u64 {
        // check if taking num * num from n is a square number
        if is_square(n - num * num) {
            // if it is return 3
            return 2;
        }
    }
    // if all else fails, the answer is 3

    return 3;
}

fn is_square(num: u64) -> bool {
    let sqrt = (num as f64).sqrt();
    let sqrt_int = sqrt as u64;
    sqrt_int * sqrt_int == num
}