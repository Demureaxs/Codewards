pub fn run_rot13() {
    // rot13("test");
    // rot13("Test");
    // rot13("Codewars");
    rot13("2nT3u2Ordj4HQvJvhyZRncrEZABAPU4cxk3C8VvtJfixERPOqZsf0n4UU9GdtkgODtXiNuaOPXwJw2RLNaEzyKxXtz4Mdx3CQShWaruBfpPKmymwX45a");
    // 2aG3h2Beqw4UDiWiulMEapeRMNONCH4pkx3P8IigWsvkRECBdMfs0a4HH9TqgxtBQgKvAhnBCKjWj2EYAnRmlXkKgm4Zqk3PDFuJnehOscCXzlzjK45n
    // 2aG3h2Beqw4UDiWiulMEapeRMNONCH4pkx3P8IigWsvkRECBdMfs0a4HH9TqgxtBQgKvAhnBCKjWj2EYAnRmlXkKgm4Zqk3PDFuJnehOscCXzlzjK45n
}

fn rot13(message: &str) -> String {
    println!("starting index {} ending index {}", 'a' as u32, 'z' as u32);
    println!(
        "starting index Caps {}, ending index caps {}",
        'A' as u32, 'Z' as u32
    );
    println!(
        "Starting index nums {}, ending index nums {}",
        '0' as u32, '9' as u32
    );
    // creates the output string
    let mut output = String::new();

    // loops through each letter in the message
    for letter in message.chars() {
        // if the letter is within the caps index
        if letter as u8 >= 65 && letter as u8 <= 90 {
            // set temp to index of letter plus 13;
            let temp = letter as u8 + 13;
            // if that number is larger than 90 (largest capital index)
            if temp > 90 {
                // push the remainder added to the start as a char
                output.push((64 + (temp % 90)) as char);
            } else {
                // otherwise push the temp
                output.push(temp as char);
            }
        // same again for lowercase
        } else if letter as u8 >= 97 && letter as u8 <= 122 {
            let temp = letter as u8 + 13;
            if temp > 122 {
                output.push((96 + (temp % 122)) as char)
            } else {
                output.push(temp as char)
            }
        // if the letter is anything else, push it as is.
        } else {
            output.push(letter);
        }
    }
    // print the output string to the console.
    println!("{}", output);
    return output;
}
