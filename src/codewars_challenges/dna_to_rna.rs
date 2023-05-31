pub fn run_dna_to_rna() {
    cw_dna_to_rna("TTTT");
    dna_to_rna("GCAT");
}

fn dna_to_rna(dna: &str) -> String {
    let mut string = String::new();

    dna.chars().for_each(|c| {
        if c == 'T' {
            string.push('U');
        } else {
            string.push(c);
        }
    });
    println!("{}", string);
    return string;
}

fn cw_dna_to_rna(dna: &str) -> String {
    // more efficient and faster...
    return dna.replace("T", "U");
}
