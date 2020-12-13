use input::input::from_assignment;

pub fn get() {
    let data = from_assignment(11);
    {
        // println!("part 1: {}", part_1(data.as_str()));
    }
    {
        println!("part 2: {}", part_2(data.as_str()));
    }
}

const OCCUPIED: i64 = 1;
const EMPTY: i64 = 2;
const FLOOR: i64 = 3;


fn get_closest_seat(input: &Vec<Vec<i64>>, row: i64, col: i64, check_col: i64, check_row: i64, rounds: i64) ->i64 {
    let mut start_col: i64 = check_col as i64 + col;
    let mut start_row: i64 = check_row as i64 + row;
    let mut current_round: i64 = 0;
    let mut current_seat = FLOOR;

    loop {
        if start_row < 0 || start_col < 0 || start_col >= input.len() as i64 || start_row >= input[0].len() as i64 {
            break;
        }
        if start_col != check_col || start_row != check_row {
            current_seat = input[start_col as usize][start_row as usize];
            if current_seat != FLOOR {
                return current_seat;
            }
        } 
        
        start_row += row;
        start_col += col;
        current_round += 1;        
        if rounds > 0 && rounds == current_round {
            break;
        }
    }

    return current_seat;
}

fn state_change(input: &Vec<Vec<i64>>, check_col: i64, check_row: i64, rounds: i64) -> (i64, bool) {
    let mut oc = 0;
    let current_seat = input[check_col as usize][check_row as usize];
    if current_seat == FLOOR {
        return (current_seat, false);
    }

    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            }
            let seat = get_closest_seat(input, i, j, check_col, check_row, rounds);
            if seat == OCCUPIED {
                oc+= 1;
            }
        }
    }

    if current_seat == EMPTY && oc == 0 {
        return (OCCUPIED, true);
    }

    if current_seat == OCCUPIED && oc >= 5 {
        return (EMPTY, true);
    }

    return (current_seat, false);
}

fn part_1(input: &str) -> i64 {
    let mut seat_plan = parse(input);
    let mut seat_plan_2 = parse(input);

    let mut changed = true;
    let row_len = seat_plan[0].len() - 1;
    let col_len = seat_plan.len() - 1;

    while changed {
        changed = false;
        for i in 1..col_len {
            for j in 1..row_len {
                let (new_val, is_new) = state_change(&seat_plan, i as i64, j as i64, 1);
                if is_new {
                    changed = true
                }
                {
                    seat_plan_2[i][j] = new_val;
                }
            }
        }
        {
            seat_plan = vec![];
            for row in &seat_plan_2 {
                let mut temp_row = vec![];
                for seat in row {
                    
                    temp_row.push(*seat);
                }
                seat_plan.push(temp_row);
            }
        }
        if !changed {
            break;
        }
    }

    let mut total_count = 0;
    for row in seat_plan {
        for seat in row {
            if seat == OCCUPIED {
                total_count+=1;
            }
        }
    }

    return total_count;
}


fn part_2(input: &str) -> i64 {
    let mut seat_plan = parse(input);
    let mut seat_plan_2 = parse(input);

    let mut changed = true;
    let row_len = seat_plan[0].len() - 1;
    let col_len = seat_plan.len() - 1;

    while changed {
        changed = false;
        for i in 1..col_len {
            for j in 1..row_len {
                let (new_val, is_new) = state_change(&seat_plan, i as i64, j as i64, 0);
                if is_new {
                    changed = true
                }
                {
                    seat_plan_2[i][j] = new_val;
                }
            }
        }
        {
            seat_plan = vec![];
            for row in &seat_plan_2 {
                let mut temp_row = vec![];
                for seat in row {
                    temp_row.push(*seat);
                }
                seat_plan.push(temp_row);
            }
        }
        if !changed {
            break;
        }
    }

    let mut total_count = 0;
    for row in seat_plan {
        for seat in row {
            if seat == OCCUPIED {
                total_count+=1;
            }
        }
    }

    return total_count;
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    let mut seats_per_row = 0;
    let mut parsed: Vec<Vec<i64>> = vec![];
    parsed.push(vec![]);
    for line in input.trim().lines() {
        if seats_per_row == 0 {
            seats_per_row = line.chars().count() + 2;
        }
        parsed.push(vec![FLOOR; seats_per_row]);
        let current_row = parsed.len() - 1;
        for (idx,chr) in line.chars().enumerate() {
            if chr == 'L' {
                parsed[current_row][idx + 1] = EMPTY;
            } else if chr == '.' {
                parsed[current_row][idx + 1] = FLOOR;
            } else if chr == '#' {
                parsed[current_row][idx + 1] = OCCUPIED;
            }
        }
    }
    parsed[0] = vec![FLOOR; seats_per_row];
    parsed.push(vec![FLOOR; seats_per_row]);
    return parsed;
}