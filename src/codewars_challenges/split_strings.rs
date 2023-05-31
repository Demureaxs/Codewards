pub fn run_solution() {
    solution("abcdef");
    solution("abcdefg");
    solution("");
}

fn solution(s: &str) -> Vec<String> {
    if s.len() < 1 {
        return vec![];
    }
    let mut result: Vec<String> = Vec::new();
    for (index, letter) in s.chars().enumerate() {
        if index % 2 == 1 {
            let mut temp = String::new();
            temp.push_str(&s.chars().collect::<Vec<char>>()[index - 1].to_string());
            temp.push_str(&letter.to_string());
            result.push(temp);
        } else if index == s.len() - 1 {
            let mut temp = String::new();
            temp.push_str(&letter.to_string());
            temp.push_str("_");
            result.push(temp);
        }
    }
    println!("{:?}", result);
    return result;
}