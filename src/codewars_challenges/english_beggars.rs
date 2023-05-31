pub fn run_beggars() {
    // beggars(&[1, 2, 3, 4, 5], 1);
    beggars(&[1, 2, 3, 4, 5], 3);
}

fn beggars(values: &[u32], n: usize) -> Vec<u32> {
    if n == 0 {
        return vec![];
    }
    // creates a vector from the array
    let temp = values.to_vec();
    // uninitialized beggar index
    let mut beggar_index: usize;
    // initialize sum to zero
    let mut sum = 0;
    // the final results vector
    let mut results: Vec<u32> = Vec::new();

    // loop through each beggar starting at zero
    for beggar in 0..n {
        // set the beggar index to the initially to the index of the beggar
        beggar_index = beggar;
        // loop through each item of temp
        for (index, item) in temp.iter().enumerate() {
            // if the beggar index matches the index
            if beggar_index == index {
                // add the item to the sum
                sum += item;
                // increase the beggar index by the number of beggars
                beggar_index += n;
            }
        }
        // push the sum to the results vector
        results.push(sum);
        // reset the sum
        sum = 0;
    }
    // return the results vector
    println!("{:?}", results);
    return results;
}