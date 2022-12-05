use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn day_template() {
    println!("Day TEMPLATE:");

    const INPUT_FILE: &str = "./input/dayTEMPLATE.txt";
    let file = File::open(INPUT_FILE).expect("Cannot open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Cannot parse line");
    }

    println!("");
}
