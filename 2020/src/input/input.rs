extern crate curl;

use std::env;
use std::fs;

pub fn read_input(filename: &str) -> String {
    return fs::read_to_string(filename).expect("Something went wrong reading file");
}

pub fn from_assignment(day: i32) -> String {
    let mut data = Vec::new();
    let mut easy = curl::easy::Easy::new();
    let session = env::var("ADVENT_SESSION").unwrap_or("".to_string());
    easy.url(&format!("https://adventofcode.com/2020/day/{}/input", day)).unwrap();

    // cookie
    let mut sess: String = "session=".to_owned();
    sess.push_str(&session);
    easy.cookie(&sess).unwrap();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    // Convert it to `String`
    let body = String::from_utf8(data).expect("body is not valid UTF8!");
    return body;
}