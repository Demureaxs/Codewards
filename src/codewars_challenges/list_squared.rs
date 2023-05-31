pub fn run_list_squared() {
    list_squared(1, 250);
    // list_squared(42, 250);
    // list_squared(300, 600);
}

fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    let mut results: Vec<(u64, u64)> = Vec::new();
    for num in m..n {
        let mut divisors: Vec<u64> = Vec::new();
        for i in 1..=(num as f64).sqrt() as u64 {
            if num % i == 0 {
                divisors.push(i.pow(2));
                if i * i != num {
                    divisors.push((num / i) * (num / i));
                }
            }
        }
        let sum: u64 = divisors.iter().sum();
        let sqrt = (sum as f64).sqrt() as u64;

        if sqrt * sqrt == sum {
            results.push((num, sum));
        }
    }
    return results;
}