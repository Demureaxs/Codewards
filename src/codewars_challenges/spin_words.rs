pub fn run_spin_words() {
    spin_words("Seriously this is the last one");
}

fn spin_words(words: &str) -> String {
    let binding = words.to_string();
    // split the string at spaces to a vector
    let word_vector: Vec<&str> = binding.split_whitespace().collect();
    // result array
    let mut result: Vec<String> = Vec::new();
    // loop through the vector
    for word in word_vector {
        // for each word, reverse it and push it to results if over 5 letters
        if word.len() > 4 {
            result.push(word.chars().rev().collect());
        } else {
            // otherwise just push the string
            result.push(word.to_string());
        }
    }
    // return results.
    println!("{:?}", result.join(" "));
    return result.join(" ");
}