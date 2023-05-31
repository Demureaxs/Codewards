pub fn run_zeros() {
    zeros(1000);
    zeros_cw(1000);
    zeros_cw(30);
    zeros(6);
}

fn zeros(n: u64) -> u64 {
    let mut count = 0;
    let mut factor = 5;

    while factor <= n {
        count += n / factor;
        factor *= 5;
    }
    // println!("{}", count);
    return count;
}

fn zeros_cw(n: u64) -> u64 {
    let n = n / 5;
    match n {
        0 => 0,
        _ => n + zeros(n),
    }
}
