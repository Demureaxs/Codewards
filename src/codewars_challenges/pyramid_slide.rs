pub fn run_longest_slide() {
    longest_slide_down(&vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]]);

    // medium
    longest_slide_down(&vec![
        vec![75],
        vec![95, 64],
        vec![17, 47, 82],
        vec![18, 35, 87, 10],
        vec![20, 4, 82, 47, 65],
        vec![19, 1, 23, 75, 3, 34],
        vec![88, 2, 77, 73, 7, 63, 67],
        vec![99, 65, 4, 28, 6, 16, 70, 92],
        vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
        vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
        vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
        vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
        vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
        vec![63, 66, 4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
        vec![4, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 4, 23],
    ]);
}

fn longest_slide_down(pyramid: &[Vec<u16>]) -> u16 {
    // get the total length of the pyramid (i.e. its height)
    let length = pyramid.len();
    // clone the sencond last pyramid
    let mut max_sum = pyramid[length - 1].clone();

    // this line allows us to work up the pyramid
    for i in (0..length - 1).rev() {
        for j in 0..pyramid[i].len() {
            let left_sum = max_sum[j];
            let right_sum = max_sum[j + 1];
            let current_sum = pyramid[i][j] + left_sum.max(right_sum);

            max_sum[j] = current_sum;
        }
    }
    println!("{}", max_sum[0]);
    return max_sum[0];
}