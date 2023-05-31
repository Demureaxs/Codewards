pub fn run_move_zeros() {
    move_zeros(&[1, 2, 0, 1, 0, 1, 0, 3, 0, 1]);
    move_zeros(&[9, 0, 0, 9, 1, 2, 0, 1, 0, 1, 0, 3, 0, 1, 9, 0, 0, 0, 0, 9]);
    move_zeros(&[0, 0]);
    move_zeros(&[0]);
    move_zeros(&[]);
}

fn move_zeros(arr: &[u8]) -> Vec<u8> {
    if arr.len() == 0 {
        return vec![];
    }

    let mut zero_vector: Vec<u8> = Vec::new();
    let mut non_zero_vector: Vec<u8> = Vec::new();

    for index in 0..arr.len() {
        if arr[index] == 0 {
            zero_vector.push(arr[index]);
        } else {
            non_zero_vector.push(arr[index]);
        }
    }

    non_zero_vector.append(&mut zero_vector);
    println!("{:?}", non_zero_vector);
    return non_zero_vector;
}
