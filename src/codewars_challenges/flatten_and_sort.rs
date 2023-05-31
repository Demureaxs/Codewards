pub fn run_flatten_and_sort() {
    flatten_and_sort(&[vec![3, 2, 1], vec![7, 9, 8], vec![6, 4, 5]]);
    flatten_and_sort(&[vec![1, 3, 5], vec![100], vec![2, 4, 6]]);
}

fn flatten_and_sort(arr: &[Vec<i32>]) -> Vec<i32> {
    let mut flat_arr: Vec<i32> = arr.concat();
    flat_arr.sort();
    return flat_arr;
}