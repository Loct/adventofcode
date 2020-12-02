use input::input::read_input;

pub fn get() {
    let file = read_input("./input/two");
    let mut valid_one = 0;
    let mut valid_two = 0;

    for line in file.trim().lines() {
        let chunks: Vec<&str> = line.split_whitespace().collect();
        if chunks.len() != 3 {
            println!("requires 3 parts {:?}", line);
        }
    
        let first: Vec<&str> =  chunks[0].split("-").collect();
        let second: Vec<&str> = chunks[1].split(":").collect();
    
        let min: usize = first[0].parse().expect("should be usize");
        let max: usize = first[1].parse().expect("should be usize");
        let needle = second[0];
        let password = chunks[2];
        if validate_count(password, needle, min, max) {
            valid_one = valid_one + 1;
        }
        if validate_position(password, needle, min - 1, max - 1) {
            valid_two = valid_two + 1;
        }
    }
    println!("answer part 1: {}", valid_one);
    println!("answer part 2: {}", valid_two)

}

fn validate_position(password: &str, needle: &str, min: usize, max: usize) -> bool {
    let chars: Vec<char> = password.chars().collect();
    if chars.len() <= max {
        return false;
    }
    let first = chars[min].to_string() == needle;
    let second = chars[max].to_string() == needle;
    (first || second) && !(first && second)
}

fn validate_count(password: &str, needle: &str, min: usize, max: usize) -> bool {
    let cnt = password.matches(needle).count();
    cnt >= min && cnt <= max 
}
