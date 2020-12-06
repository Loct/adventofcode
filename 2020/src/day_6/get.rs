use input::input::from_assignment;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn get() {
    part_1(from_assignment(6));
    part_2(from_assignment(6));

}

fn part_1(file: String) {
    let mut group_counts = 0;
    for line in file.trim().split("\n\n") {
        let cleaned = line.trim().replace("\n", "");
        let mut group_answers = HashSet::new();
        for c in cleaned.chars() {
            group_answers.insert(c);
        }
        group_counts += group_answers.len();
    }
    println!("part 1: {}", group_counts);
}

fn part_2(file: String) {
    let mut total_counts = 0;
    for line in file.trim().split("\n\n") {
        let mut group_counts = 0;
        let mut group_answers: Vec<HashMap<char, bool>> = vec!();

        let mut answers: HashSet<char> = HashSet::new();
        for user_answers in line.trim().split("\n") {
            let mut user_answer_list = HashMap::new();
            for c in user_answers.chars() {
                answers.insert(c);
                user_answer_list.insert(c, true);
            }
            group_answers.push(user_answer_list);
        }
        
        for answer in answers {
            let mut found = true;
            for user_answer in &group_answers {
                if !user_answer.contains_key(&answer) {
                    found = false;
                    break;
                };
            }
            if found {
                group_counts += 1;
            }
        }
        
        total_counts += group_counts;
    }
    println!("part 2: {}", total_counts);
}
