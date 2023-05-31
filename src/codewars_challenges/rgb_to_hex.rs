pub fn run_rgb() {
    rgb(0, 0, 0); // 000000
    rgb(1, 2, 3); // 010203
    rgb(255, 255, 255); // ffffff
    rgb(254, 253, 252); // fefdfc
    rgb(-20, 275, 125); // 00ff7d
    rgb(88, 27, 64);
}

fn rgb(r: i32, g: i32, b: i32) -> String {
    let hex_value = format!(
        "{:02X}{:02X}{:02X}",
        r.max(00).min(255),
        g.max(00).min(255),
        b.max(00).min(255)
    );
    println!("{}", hex_value);
    hex_value
}