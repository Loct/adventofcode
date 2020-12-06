use input::input::from_assignment;

pub fn get() {
    let file = from_assignment(5);
    let mut highest = 0;
    let mut seat_ids: Vec<i32> = vec!();

    for line in file.trim().lines() {
        let row = &line[..7];
        let columns = &line[7..];

        let mut upper = 127;
        let mut lower = 0;

        let mut last_char = ' ';
        for row_char in row.chars() {
            let lower_upper = get_half(row_char, lower, upper);
            lower = lower_upper.0;
            upper = lower_upper.1;
            last_char = row_char;
        }
        let row_value = get_last(last_char, lower, upper);

        upper = 7;
        lower = 0;
        last_char = ' ';
        
        for columns_char in columns.chars() {
            let lower_upper = get_half(columns_char, lower, upper);
            lower = lower_upper.0;
            upper = lower_upper.1;
            last_char = columns_char;
        }

        let columns_value = get_last(last_char, lower, upper);
        let seat_id = row_value * 8 + columns_value;
        if seat_id > highest {
            highest = seat_id;
        }
        seat_ids.push(seat_id);
    }
    println!("highest {}", highest);
    let mut start = 0;
    seat_ids.sort_by(|a, b| a.partial_cmp(b).unwrap());
    for seat_id in seat_ids {
        if start == 0 {
            start = seat_id;
            continue;
        }
        if seat_id != start + 1 {
            println!("my seat {}", start + 1);
            break;
        }
        start = seat_id;
    }
}

fn get_last(divider: char, lower: i32, upper: i32) -> i32 {
    if divider == 'F' || divider == 'L' {
        return lower;
    }
    if divider == 'B' || divider == 'R' {
        return upper;
    }
    return 0;
}
fn get_half(divider: char, lower: i32, upper: i32) -> (i32, i32) {
    if divider == 'F' || divider == 'L' {
        return (lower, ((upper + lower + 1) / 2) - 1);
    }
    if divider == 'B' || divider == 'R' {
        return (((upper + lower + 1) / 2), upper) ;
    }
    return (0, 0);
}