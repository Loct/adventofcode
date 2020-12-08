use input::input::from_assignment;
use input::input::read_input;

use std::collections::HashMap;
use std::str::FromStr;

pub fn get() {
    part_1(from_assignment(7));
    part_2(from_assignment(7));
    part_2(read_input("input/7"));
    part_2(read_input("input/7_2"));
}

fn part_1(file: String) {
    let regex_string = r"(?P<start_prefix>[a-z]+) (?P<start_color>[a-z]+) bags contain (?P<contents>.*)+";
    let regex_string_contents =
        r"(?P<number>\d+) (?P<prefix>[a-z]+) (?P<color>[a-z]+) bags?,?\.?\s?";
    let regex = regex::Regex::new(&regex_string).unwrap();
    let mut bag_refs: HashMap<String, Vec<String>> = HashMap::new();
    for line in file.trim().lines() {
        let capture_result = regex.captures(line).expect("Invalid bag line");
        let regex_contents = regex::Regex::new(&regex_string_contents).unwrap();
        for cap in regex_contents.captures_iter(line) {
            let bag_key: String = format!("{}_{}", &cap["prefix"], &cap["color"]);
            let mut current_value: Vec<String> = vec![];
            if !bag_refs.contains_key(bag_key.as_str()) {
                bag_refs.insert(bag_key, vec![]);
            }
            let bag_key: String = format!("{}_{}", &cap["prefix"], &cap["color"]);
            for bag in &bag_refs[bag_key.as_str()] {
                current_value.push(bag.to_string());
            }
            let bag_value: String = format!("{}_{}", &capture_result["start_prefix"], &capture_result["start_color"]);
            current_value.push(bag_value);
            bag_refs.insert(bag_key, current_value);
        }
    }
    let items: HashMap<String, String> = HashMap::new();
    println!("part 1: {}", seek(&bag_refs, &"shiny_gold".to_string(), items).len());
}

fn part_2(file: String) {
    let regex_string = r"(?P<start_prefix>[a-z]+) (?P<start_color>[a-z]+) bags contain (?P<contents>.*)+";
    let regex_string_contents =
        r"(?P<number>\d+) (?P<prefix>[a-z]+) (?P<color>[a-z]+) bags?,?\.?\s?";
    let regex = regex::Regex::new(&regex_string).unwrap();
    let mut bag_refs: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    for line in file.trim().lines() {
        let capture_result = regex.captures(line).expect("Invalid bag line");
        let regex_contents = regex::Regex::new(&regex_string_contents).unwrap();
        for cap in regex_contents.captures_iter(line) {
            // main bag
            let bag_key: String = format!("{}_{}", &capture_result["start_prefix"], &capture_result["start_color"]);
            // main bag should contain bags x bag_counts
            let bags: String = format!("{}_{}", &cap["prefix"], &cap["color"]);
            // count of the bags
            let bag_count: usize = FromStr::from_str(&cap["number"]).expect("bla");

            let mut current_value: Vec<(String, usize)> = vec![];
            if !bag_refs.contains_key(bag_key.as_str()) {
                bag_refs.insert(bag_key, vec![]);
            }
            let bag_key: String = format!("{}_{}", &capture_result["start_prefix"], &capture_result["start_color"]);
            for bag in &bag_refs[bag_key.as_str()] {
                current_value.push((bag.0.as_str().to_string(), bag.1));
            }
            current_value.push((bags, bag_count));
        
            bag_refs.insert(bag_key, current_value);
        }
    }
    println!("part 2 {}", count(&bag_refs, &"shiny_gold".to_string()));
}

fn count(bag_refs: &HashMap<String, Vec<(String, usize)>>, needle: &String) -> usize {
    if !bag_refs.contains_key(needle) {
        return 0
    }
    let bags = &bag_refs[needle];

    let mut num_bags: usize = 0;
    for bag in bags {
        num_bags += bag.1 + bag.1 * count(&bag_refs, &bag.0.as_str().to_string());
    }
    return num_bags;
}

fn seek(bag_refs: &HashMap<String, Vec<String>>, needle: &String, current_items: HashMap<String, String>) -> HashMap<String, String> {
    let mut cp: HashMap<String, String> = HashMap::new();
    for (key, value) in current_items {
        cp.insert(key.as_str().to_string(), value.as_str().to_string());
    }
    if !bag_refs.contains_key(needle) {
        return cp
    }
    let bags = &bag_refs[needle];
    for bag in bags {
        cp.insert(bag.to_string(), "1".to_string());
        cp = seek(&bag_refs, bag, cp);
    }
    return cp
}