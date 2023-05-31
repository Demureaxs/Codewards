pub fn run_divisors() {
    // divisors(15);
    // divisors(13);
    divisors(12);
}

fn divisors(integer: u32) -> Result<Vec<u32>, String> {
    let mut result: Vec<u32> = Vec::new();
    // loop from 0 to the number
    for i in 2..integer {
        if integer % i == 0 {
            result.push(i);
        }
        println!("integer {}, index {}, int / i {}", integer, i, integer % i);
    }
    // if integer / number = 0
    if result.is_empty() {
        return Err(format!("{} is prime", integer));
    } else {
        return Ok(result);
    }
}