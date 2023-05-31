pub fn run_validate_pin() {
    // validate_pin("234651");
    validate_pin("2346a");
}

fn validate_pin(pin: &str) -> bool {
    let filtered = pin
        .chars()
        .filter(|num| num.is_digit(10))
        .collect::<Vec<char>>();
    println!("{:?}", filtered);
    if pin.len() == 6 || pin.len() == 4 {
        if pin.len() == filtered.len() {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}