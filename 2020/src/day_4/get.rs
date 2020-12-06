use std::collections::HashMap;

use input::input::from_assignment;
 
pub fn get() {
    let file = from_assignment(4);
    let lines: Vec<&str> = file.split("\n\n").collect();
    let mut fields = HashMap::new();
    fields.insert("byr", "^19[2-9][0-9]$|^200[0-2]$");
    fields.insert("iyr", "^201[0-9]$|^2020$");
    fields.insert("eyr", "^202[0-9]$|^2030$");
    fields.insert("hgt", "^1[5-8][0-9]cm$|^19[0-3]cm$|^59in$|^6[0-9]in$|^7[0-6]in$");
    fields.insert("hcl",  "^#[0-9a-f]{6}$");
    fields.insert("ecl",  "^amb$|^blu$|^brn$|^gry$|^grn$|^hzl$|^oth$");
    fields.insert("pid",  "^\\d{9}$");

    let mut valid = 0;
    for line in lines {
        let line_split: Vec<&str> = line.split(|x| (x == ' ') || (x == '\n')).collect();
        let mut passport = HashMap::new();

        for item in line_split {
            let key_pairs: Vec<&str> = item.trim().split(":").collect();
            if key_pairs.len() != 2 {
                continue;
            }
            passport.insert(key_pairs[0], key_pairs[1]);
        }
        if validate(passport, &fields) {
            valid = valid + 1;
        }
    }
    println!{"{}", valid};
}

fn validate(passport: HashMap<&str, &str>, fields: &HashMap<&str, &str>) -> bool {
    for (key, value) in fields {
        if !passport.contains_key(key) {
            return false;
        }
        let re = regex::Regex::new(value).unwrap();
        if !re.is_match(passport.get(key).unwrap()) {
            println!("{:?}", passport.get(key).unwrap());
            return false;
        }
    }
    return true;
}