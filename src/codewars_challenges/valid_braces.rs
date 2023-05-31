pub fn run_valid_braces() {
    valid_braces("()(()))");
}

fn valid_braces(s: &str) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }

    let mut stack: Vec<char> = Vec::new();

    for brace in s.chars() {
        match brace {
            '(' | '[' | '{' => stack.push(brace),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }

        return stack.is_empty();
    }

    todo!()
}
