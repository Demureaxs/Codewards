pub fn run_expressions_matter() {
    expressions_matter(2, 1, 2);
    expressions_matter(1, 1, 1);
    expressions_matter(2, 1, 1);
    expressions_matter(1, 2, 3);
    expressions_matter(1, 3, 1);
}

fn expressions_matter(a: u64, b: u64, c: u64) -> u64 {
    let combination1 = (a + b) * c;
    let combination2 = a + b + c;
    let combination3 = a * (b + c);
    let combination4 = a * b * c;
    let combination5 = a + b * c;

    let highest = combination1
        .max(combination2.max(combination3.max(combination4.max(combination5))));
    println!("{}", highest);
    return highest;
}