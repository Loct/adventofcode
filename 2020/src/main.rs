mod input;
mod day_1;
mod day_2;
mod day_3;

use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut day_str = String::new();

    if args.len() >= 2 {
        // Get day string from args
        day_str = args[1].clone();
    } else {
        // Ask for day
        println!("Enter day: ");
        io::stdin()
            .read_line(&mut day_str)
            .expect("Failed to read line");
    }
    let day: u32 = day_str
        .trim()
        .parse()
        .expect("Day input needs to be a number");

    println!("Running day {}", day);

    match day {
        1 => day_1::get::get(),
        2 => day_2::get::get(),
        3 => day_3::get::get(),
        _ => println!("Unkown day {}", day),
    }
}

