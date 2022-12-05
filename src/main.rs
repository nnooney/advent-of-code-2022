use std::env;

mod day1;
mod day2;
mod day3;
mod day4;

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
            day => println!("Day {} not implemented", day),
        }
    }
}
