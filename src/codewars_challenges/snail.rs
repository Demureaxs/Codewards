pub fn run_snail() {
    // snail(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    // snail(&[Vec<i32>; 1] = &[Vec::new()]);
    // snail(&[vec![1]]);
    snail(&[
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 10],
        vec![11, 12, 13, 14, 15],
        vec![16, 17, 18, 19, 20],
        vec![21, 22, 23, 24, 25],
    ]);
    // 4, 4, 4, 3, 3, 2, 2, 1, 1
}

fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    if matrix.len() == 1 {
        return matrix[0].clone();
    } else if matrix.len() < 1 {
        return vec![];
    }

    let mut x_start = 0;
    let mut y_start = 0;
    let mut x_end = matrix.len() - 1;
    let mut y_end = matrix[0].len() - 1;

    let mut result: Vec<i32> = Vec::new();

    while x_start <= x_end && y_start <= y_end {
        for y in y_start..=y_end {
            result.push(matrix[x_start][y]);
        }
        x_start += 1;

        for x in x_start..=x_end {
            result.push(matrix[x][y_end]);
        }
        y_end -= 1;

        if x_start <= x_end {
            for y in (y_start..=y_end).rev() {
                result.push(matrix[x_end][y]);
            }
            x_end -= 1;
        }

        if y_start <= y_end {
            for x in (x_start..=x_end).rev() {
                result.push(matrix[x][y_start]);
            }
            y_start += 1;
        }
    }
    println!("{:?}", result);
    result
}