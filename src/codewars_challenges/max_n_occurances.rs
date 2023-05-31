use std::collections::HashMap;
// need an algorithm that will remove duplacates that appear over n times
pub fn run_delete_this() {
    // delete_nth(&[20, 37, 20, 21], 1);
    delete_nth(&[1, 1, 3, 3, 7, 2, 2, 2, 2], 3);
}

fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut counts: HashMap<u8, u32> = HashMap::new();
    let mut result: Vec<u8> = Vec::new();

    for num in lst {
        // count is a mutable reference to the entry if its not there it will be added with a value of 0
        let count = counts.entry(*num).or_insert(0);
        // if the count is less than n
        if *count < n as u32 {
            // increment the count
            *count += 1;
            // push the number to the result array
            result.push(*num)
        }
    }
    // print the result array
    println!("{:?}", result);
    // return result
    return result;
}
