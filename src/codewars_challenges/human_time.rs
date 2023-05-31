pub fn run_make_readable() {
    // make_readable(60);
    make_readable(3600);
    // make_readable(86399);
}

fn make_readable(seconds: u32) -> String {
    let hours = seconds / 3600;
    let mins = (seconds % 3600) / 60;
    let secs = (seconds % 3600) % 60;

    let time = format!("{:02}:{:02}:{:02}", hours, mins, secs);

    println!("{}", time);
    return time;
}