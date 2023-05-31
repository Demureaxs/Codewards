pub fn run_parse() {
    parse("iiisdoso");
    parse("iiisdosodddddiso");
}

fn parse(code: &str) -> Vec<i32> {
    let mut count = 0;
    let mut temp = Vec::new();

    code.chars()
        .into_iter()
        .for_each(|character| match character {
            'i' => count += 1,
            'd' => count -= 1,
            's' => count = count * count,
            'o' => temp.push(count),
            _ => println!("Something went wrong"),
        });

    temp
}
