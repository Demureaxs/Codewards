pub fn run_dont_give_me_five() {
    dont_give_me_five(4, 17);
    dont_give_me_five_cw_ver(4, 17);
}

fn dont_give_me_five(start: isize, end: isize) -> isize {
    let mut count: isize = 0;
    // loop through the range from start to the end.
    for number in start..=end {
        // if the current number contains a 5 anywhere, skip it
        if number.to_string().contains('5') {
            continue;
        }
        // otherwise add 1 to the count and continue.
        count += 1;
    }
    return count;
}

fn dont_give_me_five_cw_ver(start: isize, end: isize) -> isize {
    (start..end + 1)
        .filter(|x| !x.to_string().contains('5'))
        .count() as isize
}