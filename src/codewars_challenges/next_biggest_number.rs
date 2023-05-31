pub fn run_next_biggest_number() {
    // next_bigger_number(2017);
    // next_bigger_number(144);
    // next_bigger_number(15624168087763422193);
    // next_bigger_number(1234567890);
    // next_bigger_number(16012115342466112173);
    // next_bigger_number(2523968329512038086);
    next_bigger_number(17912558137371449165);
}

fn next_bigger_number(n: u64) -> Option<u64> {
    // return None is n is less than 10 (lowest possible number that cannot be rearranged)
    if n < 10 {
        return None;
    }
    // create a vector of digits from the number split to chars, and mapped to u8's
    let mut digits: Vec<u8> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    // establish the length of the number using digits.len()
    let len = digits.len();
    // create a tracker for the index based on length of digits
    let mut i = len - 1;
    // when the index is greater than zero and digits[before the current index] is greater than digits at index
    while i > 0 && digits[i - 1] >= digits[i] {
        // reduce the index by 1
        i -= 1;
    }
    // if the index is equal to zero return non
    if i == 0 {
        return None;
    }
    // create an inner loop variable j equal to len -1
    let mut j = len - 1;
    // while j >= i and digits j is less or equal to digits i -1
    while j >= i && digits[j] <= digits[i - 1] {
        // reduce the value of j
        j -= 1;
    }
    // swap digits i-1 with j
    digits.swap(i - 1, j);
    // reverse digits from i onward
    digits[i..].reverse();
    // fold the digits array into results
    let result = digits
        .iter()
        .fold(0u64, |acc, &digit| acc * 10 + digit as u64);
    // if the result is less than the number
    if result > n {
        // return the result
        Some(result)
    } else {
        // otherwise return None
        None
    }
}