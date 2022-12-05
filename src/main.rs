use std::env;

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} [days...]", args[0]);
        std::process::exit(1);
    }

    for day in &args[1..] {
        match &day[..] {
            "1" => day1::day_one(),
            day => println!("Day {} not implemented", day),
        }
    }
}
