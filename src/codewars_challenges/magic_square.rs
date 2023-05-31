pub fn run_magic_square() {
    magic_square(3);
    magic_square(5);
}

fn magic_square(n: u32) -> Vec<Vec<u32>> {
    let mut square = vec![vec![0 as u32; n as usize]; n as usize];
    let mut count = n * n;

    let mut vertical = 0 as usize;
    let mut horizontal = n as usize / 2;
    let mut current_number = 1;

    // square[vertical][horizontal as usize] = 1;

    while count > 0 {
        let mut vert = vertical;
        let mut hor = horizontal;
        square[vert][hor] = current_number;

        if vert > 0 {
            // move vertical up 1 (decrement)
            vert -= 1;
        } else {
            vert += n as usize - 1;
        }

        if hor < n as usize - 1 {
            hor += 1;
        } else {
            hor = 0;
        }

        if square[vert][hor] == 0 {
            vertical = vert;
            horizontal = hor;
        } else {
            if vertical < n as usize - 1 {
                vertical += 1;
            } else {
                vertical = 0;
            }
        }
        // move horizontal across 1 (increment)
        println!("{}-{}", horizontal, vertical);

        current_number += 1;
        count -= 1;
    }
    return square;
}