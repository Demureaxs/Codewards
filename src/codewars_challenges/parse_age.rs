pub fn run_parse_age() {
    get_age("2 years old");
}

fn get_age(age: &str) -> u32 {
    age.split_whitespace().collect::<Vec<&str>>()[0]
        .parse::<u32>()
        .unwrap()
}