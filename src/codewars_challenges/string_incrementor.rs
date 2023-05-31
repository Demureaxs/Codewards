pub fn run_string_incrementor() {
    increment_string("foo"); // foo1
    increment_string("foobar001"); // foobar002
    increment_string("foobar99"); // foobar100
    increment_string(""); // 1
    increment_string("foobar01");
    increment_string("qRLmFQrfd5Pv05G0PbyRtcDcgTX3LNoLqQKMEnOqfI7KfxDyhKzBzHIjjHuSh05gUmd3l4qmVPFU2Ysbsrf6ci19799269");
    increment_string("1");
}

fn increment_string(s: &str) -> String {
    let mut output = String::new();

    if s.is_empty() {
        output.push('1');
        return output;
    }

    let chars_array: Vec<char> = s.chars().collect();
    let mut index = chars_array.len() - 1;

    if index == 0 && chars_array[index].is_digit(10) {
        return (chars_array[index].to_digit(10).unwrap() + 1).to_string();
    }

    while index > 0 && chars_array[index].is_digit(10) {
        index -= 1;
    }

    if index == chars_array.len() - 1 {
        output.push_str(s);
        output.push('1');
        println!("{}", output);
        return output;
    }

    let (word, number) = s.split_at(index + 1);
    let reversed_number: String = number.chars().rev().collect();
    let mut carry = 1;

    for digit in reversed_number.chars() {
        match digit {
            '9' => {
                if carry > 0 {
                    output.push('0');
                    carry = 1;
                } else {
                    output.push('9');
                }
            }
            '0' => {
                if carry > 0 {
                    output.push(carry.to_string().chars().next().unwrap());
                    carry = 0;
                } else {
                    output.push('0');
                }
            }
            _ => {
                let digit_value = digit.to_digit(10).unwrap() + carry;
                output.push(std::char::from_digit(digit_value, 10).unwrap());
                carry = 0;
            }
        }
    }

    if carry > 0 {
        output.push(carry.to_string().chars().next().unwrap());
    }
    output.push_str(&word.chars().rev().collect::<String>());
    output.chars().rev().collect::<String>()
}
