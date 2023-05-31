pub fn run_check_exam() {
    check_exam(&["a", "a", "b", "b"], &["a", "c", "b", "d"]);
    check_exam(&["a", "a", "c", "b"], &["a", "a", "b", ""]);
    check_exam(&["a", "a", "b", "c"], &["a", "a", "b", "c"]);
    check_exam(&["b", "c", "b", "a"], &["", "a", "a", "c"]);
    check_exam_from_answers(&["b", "c", "b", "a"], &["", "a", "a", "c"]);
}

fn check_exam(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    let mut score: i64 = 0;

    for (index, element) in arr_a.iter().enumerate() {
        if element == &arr_b[index] {
            score += 4;
        } else if element != &arr_b[index] && arr_b[index] != "" {
            score -= 1;
        }
    }
    if score >= 0 {
        println!("{}", score);
        return score;
    } else {
        println!("{}", 0);
        return 0;
    }
}

use std::cmp::max;
// this version makes use of the max function in the std library to reduce if statements
fn check_exam_from_answers(arr_a: &[&str], arr_b: &[&str]) -> i64 {
    let mut score: i64 = 0;

    for (index, element) in arr_a.iter().enumerate() {
        if element == &arr_b[index] {
            score += 4;
        } else if element != &arr_b[index] && arr_b[index] != "" {
            score -= 1;
        }
    }
    return max(score, 0);
}
