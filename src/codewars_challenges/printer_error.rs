pub fn run_printer_errors() {
    printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyz");
    // 6/60

    printer_error("kkkwwwaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbmmmmmmmmmmmmmmmmmmmxyzuuuuu");
    // 11/65
}

fn printer_error(s: &str) -> String {
    let result: Vec<_> = s.chars().filter(|letter| *letter as u8 > 109).collect();
    println!("{}/{}", result.len(), s.len());
    format!("{}/{}", result.len(), s.len())
}