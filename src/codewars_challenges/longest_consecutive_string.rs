pub fn run_longest_consec() {
    longest_consec(vec!["zone", "abigail", "theta", "form", "libe", "zas"], 2); // "abigailtheta"
    longest_consec(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 3);
    // "ixoyx3452zzzzzzzzzzzz"
    longest_consec(
        vec![
            "ejjjjmmtthh",
            "zxxuueeg",
            "aanlljrrrxx",
            "dqqqaaabbb",
            "oocccffuucccjjjkkkjyyyeehh",
        ],
        1,
    );
    longest_consec(vec![], 3);
}

fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    if strarr.len() == 0 || strarr.len() < k {
        return "".to_string();
    }

    // temporary longest concatenation
    let mut length = 0;

    let mut longest_concat = String::new();

    // loop through the n length -1
    for index in 0..=strarr.len() - k {
        let mut temp_longest = String::new();
        for j in index..index + k {
            temp_longest.push_str(strarr[j]);
        }

        if temp_longest.len() > length {
            longest_concat = temp_longest;
            length = longest_concat.len();
        } else {
            continue;
        }
    }
    println!("{}", longest_concat);
    return longest_concat;
}
