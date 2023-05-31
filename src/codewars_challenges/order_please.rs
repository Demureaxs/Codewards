use std::collections::HashMap;

pub fn run_order() {
    order("is2 Thi1s T4est 3a");
    order_codewars("is2 Thi1s T4est 3a");
}

fn order(sentence: &str) -> String {
    if sentence.len() < 1 {
        return "".to_string();
    }
    let mut word_pos: HashMap<&str, u32> = HashMap::new();

    for word in sentence.split(' ') {
        for letter in word.chars() {
            if letter.is_ascii_digit() {
                word_pos.insert(word, letter.to_digit(10).unwrap());
            }
        }
    }
    let mut entries: Vec<_> = word_pos.iter().collect();
    entries.sort_by_key(|entry| entry.1);
    let result: Vec<&str> = entries.iter().map(|item| *item.0).collect();

    println!("{:?}", result.join(" "));
    return result.join(" ");
}

fn order_codewars(sentence: &str) -> String {
    // split the word at whitespace and map to strings, collect into a vector
    let mut words: Vec<_> = sentence.split_whitespace().map(String::from).collect();
    // sort the words by key,
    // split into characters and find the number in the word,
    // sort by that number and unwrap
    words.sort_by_key(|word| word.chars().find(|letter| letter.is_digit(10)).unwrap());

    // return words joined
    println!("{}", words.join(" "));
    words.join(" ")
}
