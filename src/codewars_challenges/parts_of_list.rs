pub fn run_part_list() {
    part_list(vec!["I", "wish", "I", "hadn't", "come"]);
}

fn part_list(arr: Vec<&str>) -> String {
    let mut result = String::new();
    for index in 1..arr.len() {
        let (start, end) = arr.split_at(index);
        result.push_str("(");
        result.push_str(&start.join(" "));
        result.push_str(", ");
        result.push_str(&end.join(" "));
        result.push_str(")")
    }

    println!("{}", result);
    return result;
}