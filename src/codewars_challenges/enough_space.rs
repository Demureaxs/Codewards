pub fn run_enough() {
    enough(10, 5, 5);
    enough(100, 60, 50);
    enough(20, 5, 5);
}

fn enough(cap: i32, on: i32, wait: i32) -> i32 {
    if cap - on >= wait {
        println!("{}", 0);
        return 0;
    } else {
        println!("{}", wait - (cap - on));
        return wait - (cap - on);
    }
}