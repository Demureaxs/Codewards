pub fn run_desc_order() {
    descending_order(1254859723);
    descending_order(145263);
    descending_order(123456789);
    descending_order(0);
}

fn descending_order(x: u64) -> u64 {
    let mut digits: Vec<char> = x.to_string().chars().collect();
    digits.sort_by(|a, b| b.cmp(a));
    let output = digits.iter().collect::<String>().parse::<u64>().unwrap();

    println!("{}", output);
    return output;
}