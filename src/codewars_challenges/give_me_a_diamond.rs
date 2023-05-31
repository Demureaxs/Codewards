pub fn run_print() {
    print(3);
    // print(5);
    // print(-3);
    print_cw(1);
}

fn print(n: i32) -> Option<String> {
    if n < 0 || n % 2 == 0 {
        return None;
    }

    let mut temp_string = String::new();

    for index in 1..=n {
        if index % 2 == 0 {
            continue;
        }
        temp_string.push_str(&" ".repeat(((n - index) / 2) as usize));
        temp_string.push_str(&"*".repeat(index as usize));
        temp_string.push_str("\n");
    }

    for index in (1..=n - 2).rev() {
        if index % 2 == 0 {
            continue;
        }
        temp_string.push_str(&" ".repeat(((n - index) / 2) as usize));
        temp_string.push_str(&"*".repeat(index as usize));
        temp_string.push_str("\n");
    }
    println!("{}", temp_string);
    Some(temp_string)
}

fn print_cw(n: i32) -> Option<String> {
    if n < 0 || n % 2 == 0 {
        return None;
    }

    let n = n as usize;
    let diamond = (1..=n)
        .chain((1..n).rev())
        .step_by(2)
        .map(|i| format!("{}{}\n", " ".repeat((n - i) / 2), "*".repeat(i)))
        .collect();

    Some(diamond)
}
