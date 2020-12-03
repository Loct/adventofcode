mod input;
mod one;
mod two;
mod three;

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
        1 => one::one::get(),
        2 => two::two::get(),
        3 => three::three::get(),
        _ => println!("Unkown day {}", day),
    }
}

