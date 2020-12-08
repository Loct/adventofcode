use input::input::from_assignment;

pub fn get() {
    let data = from_assignment(8);
    {
        part_1(data.as_str());

    }
    {
        part_2(data.as_str());
    }
}

fn part_1(input: &str) {
    let mut ops: Vec<(String, i64)> = vec![];
    for line in input.trim().lines() {
        let split: Vec<&str> = line.split(" ").collect();
        let op = split[0];
        let value_str: String = split[1].replace("+", "");
        let value: i64 = value_str.parse().expect("not ok");
        ops.push((op.to_string(), value));
    }
    let mut acc: i64 = 0;
    let mut pos: usize = 0;
    let mut visited: Vec<usize> = vec![];
    
    while !visited.contains(&pos) {
        visited.push(pos);
        let (op, value) = &ops[pos];
        let res = execute(op.as_str(), *value, pos, acc);
        acc = res.0;
        pos = res.1;
    }    
    println!("part 1: {}", acc);
}

fn part_2(input: &str) {
    let mut ops: Vec<(String, i64)> = vec![];
    for line in input.trim().lines() {
        let split: Vec<&str> = line.split(" ").collect();
        let op = split[0];
        let value_str: String = split[1].replace("+", "");
        let value: i64 = value_str.parse().expect("not ok");
        ops.push((op.to_string(), value));
    }
    
    let mut found = false;
    let mut repl = 0; // replace the 0th element which is nop or jmp
    while !found {
        let replaced = replace(&ops, repl);
        let mut acc: i64 = 0;
        let mut pos: usize = 0;
        let mut visited: Vec<usize> = vec![];
        while !visited.contains(&pos) {
            visited.push(pos);
            let (op, value) = &replaced[pos];
            let res = execute(op.as_str(), *value, pos, acc);
            acc = res.0;
            pos = res.1;
            if pos == ops.len() {
                found = true;
                break;
            }
        }
    
        if found {
            println!("part 2: {}", acc);
            break;
        } else {
            repl += 1;
            if repl == ops.len() as i32 {
                println!("not found {}", repl);
                break
            }
        }
    }
}

fn replace(org: &Vec<(String, i64)>, replace_op: i32) -> Vec<(String, i64)> {
    let deref = org;
    let mut replaced: Vec<(String, i64)> = vec![];
    let mut current_replace = 0;
    for (op, value) in deref {
        let mut new_op = op.as_str();
        if op == "nop" || op == "jmp" {
            if current_replace == replace_op {
                if op == "jmp" {
                    new_op = "nop";
                } else {
                    new_op = "jmp";
                }
            }
            current_replace += 1;
        }
        replaced.push((new_op.to_string(), *value));
    }
    return replaced;
}

fn execute(instr: &str, value: i64, current: usize, acc: i64) -> (i64, usize) {
    if instr == "nop" {
        return (acc, current + 1);
    }
    if instr == "acc" {
        return (acc + value, current + 1);
    }
    if instr == "jmp" {
        return (acc, ((current as i64) + value) as usize);
    }
    return (0, 0);
}