use input::input::from_assignment;

pub fn get() {
    let data = from_assignment(9);
    {
        println!("part 1: {}", part_1(data.as_str()));
    }
    {
        part_2(data.as_str());
    }
}

fn part_1(input: &str) -> i64 {
    let numbers = parse(input);
    let offset = 25;
    for  x in offset..numbers.len() {
        let temp = numbers[x-offset..x].to_vec();
        if !sum_multiply(&temp, numbers[x], 2, 0) {
            return numbers[x];
        }
    }
    return 0;
}

fn part_2(input: &str) {
    let invalid_number = part_1(input);
    let numbers = parse(input);
    for offset in 0..numbers.len() {
        let temp = numbers[offset..].to_vec();
        let (found, mut summed) = sum_contiguous(&temp, invalid_number);
        if found {
            summed.sort();
            println!("part 2: {} + {} = {}", summed[0], summed[summed.len() - 1], summed[0] + summed[summed.len() - 1]);
            break;
        }
    }
}

fn sum_contiguous(input: &Vec<i64>, needle: i64) -> (bool, Vec<i64>) {
    let mut start: i64 = 0;
    let mut numbers: Vec<i64> = vec![];
    for value in input {
        start += *value;
        numbers.push(*value);
        if start == needle {
            return (true, numbers);
        }
        if start > needle {
            return (false, numbers);
        }
    }
    return (false, numbers);
}

fn parse(input: &str) -> Vec<i64> {
    let mut res: Vec<i64> = vec![];
    for value in input.trim().lines() {
        let number_one :i64 = value.parse().expect("number expected");
        res.push(number_one);
    }
    return res;
}

fn sum_multiply(input: &Vec<i64>, needle: i64, numbers :i8, current_value :i64) -> bool {
    // done no need to loop again.
    if numbers == 0 {
        return false;
    }
    for value in input{
        let number_one :i64 = *value;
        // store the addition
        let new_addition = current_value + number_one;
        // recurse
        if new_addition < needle  && numbers > 1 {
            // if answer of the multiple signals true, then done
            if sum_multiply(input, needle, numbers - 1, new_addition) {
                return true;
            }
        }
        // right answer should happen at the lowest level
        // print and return the answer is done.
        if new_addition == needle && numbers == 1 {
            return true;
        }
    }
    return false;
}