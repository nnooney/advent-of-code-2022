use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn day_four() {
    println!("Day 4:");

    const INPUT_FILE: &str = "./input/day4.txt";
    let file = File::open(INPUT_FILE).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut sum_full_overlaps = 0;
    let mut sum_partial_overlaps = 0;
    for line in reader.lines() {
        let line = line.expect("Cannot parse line");

        let parts: Vec<&str> = line.split(&['-', ',']).collect();

        if ranges_fully_overlap(&parts) {
            sum_full_overlaps += 1;
        }

        if ranges_partially_overlap(&parts) {
            sum_partial_overlaps += 1;
        }
    }

    println!("Number of pairs that fully overlap: {}", sum_full_overlaps);
    println!(
        "Number of pairs that partially overlap: {}",
        sum_partial_overlaps
    );

    println!("");
}

fn ranges_fully_overlap(parts: &Vec<&str>) -> bool {
    let parts: Vec<i32> = parts
        .iter()
        .map(|s| s.parse::<i32>().expect("Not a number"))
        .collect();

    // If either the start or end sections are the same, then they fully overlap
    if parts[0] == parts[2] || parts[1] == parts[3] {
        return true;
    }
    // Otherwise the ranges fully overlap if the same range has both the smallest
    // and largest section.
    else if (parts[0] < parts[2]) == (parts[1] > parts[3]) {
        return true;
    }

    return false;
}

fn ranges_partially_overlap(parts: &Vec<&str>) -> bool {
    let parts: Vec<i32> = parts
        .iter()
        .map(|s| s.parse::<i32>().expect("Not a number"))
        .collect();

    // The ranges do not overlap if the end of one is smaller than the beginning
    // of another.
    return !(parts[1] < parts[2] || parts[3] < parts[0]);
}
