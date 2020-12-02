use input::input::read_input;

pub fn get() {
    let file = read_input("./input/one");
    print!("answer part 1: ");
    sum_multiply(&file, 2020, 2, 0, 1);
    print!("answer part 2: ");
    sum_multiply(&file, 2020, 3, 0, 1);
}

fn sum_multiply(input: &str, needle: i32, numbers :i8, current_value :i32, multiplication: i32) -> bool {
    // done no need to loop again.
    if numbers == 0 {
        return false;
    }
    for value in input.trim().lines() {
        let number_one :i32 = value.parse().expect("number expected");
        // store the addition
        let new_addition = current_value + number_one;
        let new_multiplication = multiplication * number_one;
        // recurse
        if new_addition < needle  && numbers > 1 {
            // if answer of the multiple signals true, then done
            if sum_multiply(input, needle, numbers - 1, new_addition, new_multiplication) {
                return true;
            }
        }
        // right answer should happen at the lowest level
        // print and return the answer is done.
        if new_addition == needle && numbers == 1 {
            println!("{}", new_multiplication);
            return true;
        }
    }
    return false;
}