use input::input::from_assignment;
 
pub fn get() {
    let file = from_assignment(3);
    let mut lines: Vec<Vec<char>> = vec!();
    for line in file.trim().lines() {
        let chars: Vec<char> = line.chars().collect();
        lines.push(chars);
    }
    let mut result = count(&lines, 1, 1);
    result = result * count(&lines, 3, 1);
    result = result * count(&lines, 5, 1);
    result = result *  count(&lines, 7, 1);
    result = result *  count(&lines, 1, 2);
    println!("result: {:?}", result)

}

fn count(lines: &Vec<Vec<char>>, right: usize, down: usize) -> i64 {
    let mut it_right: usize = 0;
    let mut it_down: usize = 0;
    let len_lines = lines.len();
    let mut trees = 0;
    
    while it_down < len_lines {
        if get_char(lines, it_right, it_down) == '#' {
            trees = trees + 1;
        }
        it_right = it_right + right;
        it_down = it_down + down;
    }

    return trees;
}

fn get_char(lines: &Vec<Vec<char>>, x: usize, y: usize) -> char {
    let norm_x = x % lines[0].len();
    return lines[y][norm_x];
}