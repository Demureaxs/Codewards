pub fn run_expanded_form() {
    expanded_form(12);
    expanded_form(42);
    expanded_form(70304);
}

fn expanded_form(n: u64) -> String {
    let temp_string = n.to_string();
    let length = n.to_string().len();
    let mut output = String::new();

    for (index, num) in temp_string.chars().enumerate() {
        if num == '0' {
            continue;
        } else {
            output.push(num);
            output.push_str(&"0".repeat(length - (index + 1)));
            output.push_str(" + ");
        }
    }

    let (output2, _) = output.split_at(output.len() - 3);

    println!("{:?}", output2);
    return output2.to_string();
}