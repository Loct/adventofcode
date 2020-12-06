use input::input::from_assignment;

pub fn get() {
    let file = from_assignment(5);
    let mut highest = 0;
    let mut seat_ids: Vec<i32> = vec!();

    for line in file.trim().lines() {
        let row = &line[..7];
        let columns = &line[7..];

        let row_value = divide(row, 127) * 8;
        let col_value = divide(columns, 7);
        let seat_id = row_value + col_value;
        if seat_id > highest {
            highest = seat_id;
        }
        seat_ids.push(seat_id);
    }
    seat_ids.sort();
    println!("highest {}", highest);
    println!("my seat {}", get_seat(seat_ids));
}

fn divide(input: &str, max: i32) -> i32 {
    let mut lower = 0;
    let mut upper = max;
    let mut last_char = ' ';
    for mutator in input.chars() {
        let lower_upper = get_half(mutator, lower, upper);
        lower = lower_upper.0;
        upper = lower_upper.1;
        last_char = mutator;
    }
    let row_value = get_last(last_char, lower, upper);
    return row_value;
}

fn get_seat(seat_ids: Vec<i32>) -> i32 {
    let mut start = 0;
    for seat_id in seat_ids {
        if start == 0 {
            start = seat_id;
            continue;
        }
        if seat_id != start + 1 {
            return start + 1;
        }
        start = seat_id;
    }
    return 0;
}

fn get_last(divider: char, lower: i32, upper: i32) -> i32 {
    if divider == 'F' || divider == 'L' {
        return lower;
    }
    return upper;
}

fn get_half(divider: char, lower: i32, upper: i32) -> (i32, i32) {
    if divider == 'F' || divider == 'L' {
        return (lower, ((upper + lower + 1) / 2) - 1);
    }
    return (((upper + lower + 1) / 2), upper) ;
}