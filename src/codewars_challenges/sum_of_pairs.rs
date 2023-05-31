pub fn run_sum_of_pairs() {
    sum_pairs(&[1, 4, 8, 7, 3, 15], 8); // some(1 / 7)
}
use std::collections::HashSet;

fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    let mut seen = HashSet::new();

    for num in ints {
        let difference = s - *num;
        if seen.contains(&difference) {
            return Some((difference, *num));
        }
        seen.insert(num);
    }

    None
}