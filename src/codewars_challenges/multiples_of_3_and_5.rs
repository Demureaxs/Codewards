pub fn run_solution() {
    solution(20);
    solution(40);
    solution(4);
    solution_cw2(4);
}

fn solution(num: i32) -> i32 {
    // create a temporary array
    let mut temp: Vec<i32> = Vec::new();

    // loop for the range of the num and if its divisible by 3 or 5
    for index in 0..num {
        if index % 3 == 0 || index % 5 == 0 {
            temp.push(index)
        }
    }

    let result = temp.iter().fold(0, |acc, cv| acc + cv);

    return result;
}

fn solution_cw2(num: i32) -> i32 {
    (3..num).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}
