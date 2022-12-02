use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    day_one();
}

fn day_one() {
    const DAY_ONE_FILE: &str = "./input/day1.txt";
    let file = File::open(DAY_ONE_FILE).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut vec: Vec<u64> = Vec::new();
    let mut sum: u64 = 0;
    for line in reader.lines() {
        let line = line.expect("Cannot parse line");

        if line == "" {
            vec.push(sum);
            sum = 0;
        } else {
            sum += line
                .trim()
                .parse::<u64>()
                .expect("Cannot parse number from line");
        }
    }

    match vec.iter().max() {
        Some(max) => println!("Max: {}", max),
        None => println!("Empty vector"),
    }

    vec.sort_by(|a, b| b.cmp(a));
    println!("Sum of top 3: {}", vec[0] + vec[1] + vec[2])
}
