pub fn run_duplicate_encoder() {
    duplicate_encode("Success");
}

fn duplicate_encode(word: &str) -> String {
    // loop through the word
    let mut result = String::new();
    // create a hashmap
    for (index, letter) in word.to_lowercase().chars().enumerate() {
        let mut multiple_appearances = false;
        // use entry.or_insert to keep a counter for each letter
        for (i, ch) in word.to_lowercase().chars().enumerate() {
            if i != index && ch == letter {
                multiple_appearances = true;
            }
        }
        if multiple_appearances {
            result.push_str(")");
        } else {
            result.push_str("(");
        }
    }
    println!("{}", result);
    todo!()
}