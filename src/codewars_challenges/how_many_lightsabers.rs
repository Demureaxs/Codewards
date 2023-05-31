pub fn run_lightsabers() {
    how_many_lightsabers_do_you_own("");
    how_many_lightsabers_do_you_own("Adam");
    how_many_lightsabers_do_you_own("Zach");
    how_many_lightsabers_do_you_own("zach");
}

fn how_many_lightsabers_do_you_own(name: &str) -> u8 {
    match name {
        "Zach" => 18,
        _ => 0,
    }
}