use input::input::from_assignment;

pub fn get() {
    let data = from_assignment(10);
    {
        println!("part 1: {}", part_1(data.as_str()));
    }
    {
        println!("part 2: {}", part_2(data.as_str()));
    }
}

fn part_1(input: &str) -> i64 {
    let numbers = parse(input);

    let mut one_differences = 0; 
    let mut three_differences = 1;

    let mut previous_number = 0;
    for number in numbers {
        if previous_number + 3 == number {
            three_differences += 1;
        }
        if previous_number + 1 == number {
            one_differences += 1;
        }
        previous_number = number;
    }
    
    println!("{} {}", one_differences, three_differences);
    return one_differences * three_differences;
}

fn part_2(input: &str) -> usize {
    let numbers = parse(input);
    let numbers_len = numbers.len();
    let mut counts: Vec<usize> = vec![0; numbers_len];
    counts[0] = 1;


    for (idx, number) in numbers.iter().enumerate() {
        if numbers_len <= idx {
            break;
        }
        for j in 1..4 {
            if numbers_len <= idx + j {
                break;
            }
            if numbers[idx + j] - number <= 3 {
                counts[idx + j] += counts[idx];
            }
        }
    }

    return counts[numbers_len - 1]
}

fn parse(input: &str) -> Vec<i64> {
    let mut res: Vec<i64> = vec![];
    res.push(0);
    for value in input.trim().lines() {
        let number_one :i64 = value.parse().expect("number expected");
        res.push(number_one);
    }
    res.sort();
    return res;
}