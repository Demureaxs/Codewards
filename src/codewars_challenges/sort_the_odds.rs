pub fn run_sort_array() {
    sort_array(&[5, 3, 2, 8, 1, 4]);
    // sort_array(&[5, 3, 1, 8, 0]);
}

fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut odds_index: Vec<(usize, i32)> = Vec::new();
    let mut result: Vec<i32> = Vec::new();

    for (index, number) in arr.clone().iter().enumerate() {
        if number % 2 != 0 {
            odds_index.push((index, *number));
        }
    }

    odds_index.sort_by(|a, b| a.0.cmp(&b.0));
    let mut odds_values = odds_index.clone();
    odds_values.sort_by(|a, b| a.1.cmp(&b.1));

    for (index, number) in arr.clone().iter().enumerate() {
        let mut number_pushed = false;
        for (ind, (pos, _)) in odds_index.iter().enumerate() {
            if index == *pos {
                result.push(odds_values[ind].1);
                number_pushed = true;
                continue;
            }
        }
        if number_pushed {
            continue;
        } else {
            result.push(*number);
        }
    }

    println!("{:?}", result);
    return result;
}
