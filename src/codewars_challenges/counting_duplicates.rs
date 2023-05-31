pub fn run_count_duplicates() {
    count_duplicates_b("indivisibility");
    count_duplicates("abcdea");
}

use std::collections::HashMap;

fn count_duplicates(text: &str) -> u32 {
    let mut word_map: HashMap<char, u32> = HashMap::new();

    for letter in text.to_lowercase().chars() {
        *word_map.entry(letter).or_insert(0) += 1;
    }

    let results: Vec<&char> = word_map
        .iter()
        .filter(|item| *item.1 > 1)
        .map(|item| item.0)
        .collect();
    println!("{}", results.len());
    return results.len() as u32;
}

fn count_duplicates_b(text: &str) -> u32 {
    let mut word_map: HashMap<char, u32> = HashMap::new();
    for letter in text.to_lowercase().chars() {
        *word_map.entry(letter).or_insert(0) += 1;
    }
    return word_map.values().filter(|&&count| count > 1 as u32).count() as u32;
}