pub fn run_count_bits() {
    count_bits(1234);
    count_bits_cw(1234);
}

fn count_bits(n: i64) -> u32 {
    let bits = format!("{:b}", n)
        .to_string()
        .chars()
        .filter(|num| *num == '1')
        .collect::<Vec<char>>()
        .len() as u32;

    return bits;
}

fn count_bits_cw(n: i64) -> u32 {
    n.count_ones()
}