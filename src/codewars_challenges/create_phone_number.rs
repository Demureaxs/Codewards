pub fn run_create_phone_number() {
    create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    // create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
    // create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]);
}

fn create_phone_number(numbers: &[u8]) -> String {
    numbers
        .iter()
        .enumerate()
        .map(|(index, num)| match index {
            0 => format!("({}", *num),
            2 => format!("{}) ", *num),
            6 => format!("-{}", *num),
            _ => format!("{}", *num),
        })
        .collect::<Vec<String>>()
        .join("")
}