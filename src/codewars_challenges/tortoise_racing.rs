pub fn run_race() {
    race(105, 255, 84);
}

fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    if v1 >= v2 {
        return None;
    }

    let b_advantage = v2 - v1;
    let time_to_close_gap = (g * 3600) / b_advantage;

    let hours = time_to_close_gap / 3600;
    let mins = (time_to_close_gap % 3600) / 60;
    let seconds = time_to_close_gap % 60;

    println!("{}{}{}", hours, mins, seconds);

    Some(vec![hours, mins, seconds])
}