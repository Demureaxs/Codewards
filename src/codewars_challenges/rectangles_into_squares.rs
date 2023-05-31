pub fn run_sq_in_rect() {
    sq_in_rect(5, 3); // should be vec![3,2,1,1]
    sq_in_rect(3, 5); // should be vec![3,2,1,1]
    sq_in_rect(5, 5); // should be none
}

fn sq_in_rect(lng: i32, wdth: i32) -> Option<Vec<i32>> {
    if lng == wdth {
        return None;
    }
    let smallest_side = lng.min(wdth);
    let mut largest_side = lng.max(wdth);
    let mut remaining_area = smallest_side * largest_side;
    let mut squares: Vec<i32> = Vec::new();

    for num in (0..=smallest_side).rev() {
        let square_size = num;
        squares.push(square_size);
        let square_size_area = square_size * square_size;
        remaining_area -= square_size_area;
        largest_side -= smallest_side;

        if num * num == remaining_area {
            squares.push(num);
            remaining_area -= num;
            break;
        }
    }

    if remaining_area > 0 {
        return None;
    }

    println!("{:?}", squares);
    Some(squares)
}