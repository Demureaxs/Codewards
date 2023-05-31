pub fn run_sum_difference() {
    sum_of_differences(&[1, 2, 10]);
    sum_of_differences(&[-17, 17]);
    sum_of_differences(&[1, 1, 1, 1, 1]);
}

fn sum_of_differences(arr: &[i8]) -> Option<i8> {
    if arr.len() < 2 {
        return None;
    } else {
        let mut sorted_arr = arr.to_vec();
        sorted_arr.sort_by(|a, b| b.cmp(a));

        let mut sum = 0;
        for i in 0..sorted_arr.len() - 1 {
            sum += sorted_arr[i] - sorted_arr[i + 1];
        }
        return Some(sum);
    }
}
