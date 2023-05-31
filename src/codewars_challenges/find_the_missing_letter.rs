pub fn run_find_missing_letter() {
    find_missing_letter(&['a', 'b', 'c', 'd', 'f']);

    find_missing_letter_2(&['O', 'Q', 'R', 'S']);
}

fn find_missing_letter(chars: &[char]) -> char {
    let mut next_ascii_char = chars[1] as u32;
    for (_, letter) in chars.iter().enumerate() {
        if *letter as u32 != next_ascii_char - 1 {
            return std::char::from_u32(next_ascii_char - 1).unwrap();
        }
        next_ascii_char += 1;
    }
    todo!()
}

fn find_missing_letter_2(chars: &[char]) -> char {
    (chars
        .windows(2)
        .map(|w| (w[0] as u8, w[1] as u8))
        .find(|&w| w.0 + 1 != w.1)
        .unwrap()
        .0
        + 1) as char //Add 1 to previous character, to get the correct character.
}