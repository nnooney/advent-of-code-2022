use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn day_three() {
    println!("Day 3:");

    const INPUT_FILE: &str = "./input/day3.txt";
    let file = File::open(INPUT_FILE).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut sum_compartments = 0;
    let mut sum_badges = 0;
    let mut group_knapsacks: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Cannot parse line");

        sum_compartments += check_compartments(&line);

        group_knapsacks.push(line);
        if group_knapsacks.len() == 3 {
            sum_badges += check_groups(&group_knapsacks);
            group_knapsacks.clear();
        }
    }

    println!("Sum of priorities in compartments is {}", sum_compartments);
    println!("Sum of priorities in group badges is {}", sum_badges);

    println!("");
}

fn check_groups(knapsacks: &Vec<String>) -> u32 {
    for char in knapsacks[0].chars() {
        if knapsacks[1].contains(char) && knapsacks[2].contains(char) {
            return calculate_priority(char);
        }
    }

    return 0;
}

fn check_compartments(knapsack: &str) -> u32 {
    let compartment_one = &knapsack[0..knapsack.len() / 2];
    let compartment_two = &knapsack[knapsack.len() / 2..];

    for char in compartment_one.chars() {
        if compartment_two.contains(char) {
            return calculate_priority(char);
        }
    }

    return 0;
}

fn calculate_priority(char: char) -> u32 {
    if char.is_ascii_uppercase() {
        return (char as u32) - ('A' as u32) + 27;
    } else if char.is_ascii_lowercase() {
        return (char as u32) - ('a' as u32) + 1;
    }
    return 0;
}
