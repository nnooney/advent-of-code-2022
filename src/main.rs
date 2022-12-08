use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} [days...]", args[0]);
        std::process::exit(1);
    }

    for day in &args[1..] {
        match &day[..] {
            "1" => day1::day_one(),
            "2" => day2::day_two(),
            "3" => day3::day_three(),
            "4" => day4::day_four(),
            "5" => day5::day_five(),
            "6" => day6::day_six(),
            "7" => day7::day_seven(),
            day => println!("Day {} not implemented", day),
        }
    }
}
