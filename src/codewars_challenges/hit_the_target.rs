pub fn run_solution() {
    // solution(&[vec!['>', ' '], vec![' ', 'x']]);

    solution(&[
        vec![' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', '>', ' ', 'x'],
        vec![' ', ' ', ' ', ' ', ' '],
    ]);
}

fn solution(mtrx: &[Vec<char>]) -> bool {
    let mut arr_i: (usize, usize) = (0, 0);
    let mut x_i: (usize, usize) = (0, 0);

    let mut direction: char = '^';

    for (x_index, _) in mtrx.iter().enumerate() {
        for (y_index, sym) in mtrx[x_index].iter().enumerate() {
            if ['^', '>', 'v', '<'].contains(sym) {
                arr_i = (x_index, y_index);
                direction = *sym;
            } else if *sym == 'x' {
                x_i = (x_index, y_index);
            }
        }
    }
    println!("arrow location {:?} cross location {:?}", arr_i, x_i);
    println!("{}", direction);
    match direction {
        '^' => {
            if arr_i.1 == x_i.1 && arr_i.0 > x_i.0 {
                return true;
            }
        }
        '>' => {
            if arr_i.0 == x_i.0 && arr_i.1 < x_i.1 {
                return true;
            }
        }
        'v' => {
            if arr_i.1 == x_i.1 && arr_i.0 < x_i.0 {
                return true;
            }
        }
        '<' => {
            if arr_i.0 == x_i.0 && arr_i.1 > x_i.1 {
                return true;
            }
        }
        _ => (),
    }
    return false;
}