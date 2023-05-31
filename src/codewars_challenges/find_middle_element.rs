pub fn run_gimme() {
    gimme([2, 3, 1]);
    gimme([-2, -3, -1]);
}

fn gimme(input_array: [i32; 3]) -> usize {
    let mut temp = input_array.clone();
    temp.sort();
    println!("{:?}", temp);

    // find index of temp[1] and return it
    for (index, number) in input_array.iter().enumerate() {
        if *number == temp[1] {
            return index;
        }
    }
    todo!()
}