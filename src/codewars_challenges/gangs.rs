pub fn run_gangs() {
    gangs(&[2, 3, 6, 5], 15);
    gangs(&[2, 3], 6);
}

use std::collections::HashSet;

fn gangs(divisors: &[u32], k: u32) -> u32 {
    let mut result: HashSet<Vec<u32>> = HashSet::new();
    for num in 1..=k {
        let mut temp: Vec<u32> = Vec::new();
        for div in divisors {
            if num % div == 0 {
                temp.push(*div);
            }
        }
        result.insert(temp);
    }
    result.len() as u32
}
