mod input;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;

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
        4 => day_4::get::get(),
        5 => day_5::get::get(),
        6 => day_6::get::get(),
        7 => day_7::get::get(),
        8 => day_8::get::get(),
        9 => day_9::get::get(),
       10 => day_10::get::get(),
        _ => println!("Unkown day {}", day),
    }
}

