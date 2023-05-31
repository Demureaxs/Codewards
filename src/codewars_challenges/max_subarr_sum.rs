pub fn run_max_subarr_sum() {
    max_sequence(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]); // 6

    max_sequence(&[11]); // 11

    max_sequence(&[-32]); // 0
    
    max_sequence_cw_solution(&[-32]); // 0
}

fn max_sequence(seq: &[i32]) -> i32 {
    let mut max_so_far = 0;
    let mut max_ending_here = 0;

    for num in seq {
        max_ending_here += num;
        if max_so_far < max_ending_here {
            max_so_far = max_ending_here;
        }
        if max_ending_here < 0 {
            max_ending_here = 0;
        }
    }
    return max_so_far;
}

fn max_sequence_cw_solution(seq: &[i32]) -> i32 {
    let mut max_sum = 0;
    let mut tmp_sum = 0;
    for x in seq {
        tmp_sum = (tmp_sum + x).max(0);
        max_sum = max_sum.max(tmp_sum);
    }
    max_sum
}