pub fn run_multiplication_table() {
    multiplication_table(3);
}

fn multiplication_table(len: usize) -> Vec<Vec<usize>> {
    let mut output: Vec<Vec<usize>> = vec![];

    for num in 1..=len {
        let mut temp_vec: Vec<usize> = Vec::new();

        for num2 in 1..=len {
            temp_vec.push(num * num2);
        }

        output.push(temp_vec);
    }
    // println!("{:?}", output);
    return output;
}