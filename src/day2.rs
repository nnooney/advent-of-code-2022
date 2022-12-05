use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn day_two() {
    println!("Day 2:");

    // Precompute the possible outcomes and their associated scores.
    let scores_choice: HashMap<&str, i32> = HashMap::from([
        ("A X", 4), // 1 for rock, 3 for draw
        ("A Y", 8), // 2 for paper, 6 for win
        ("A Z", 3), // 3 for scissors, 0 for lose
        ("B X", 1), // 1 for rock, 0 for lose
        ("B Y", 5), // 2 for paper, 3 for draw
        ("B Z", 9), // 3 for scissors, 6 for win
        ("C X", 7), // 1 for rock, 6 for win
        ("C Y", 2), // 2 for paper, 0 for lose
        ("C Z", 6), // 3 for scissors, 3 for draw
    ]);

    // It turns out X Y Z is not our choice, but the result (lose, draw, win).
    let scores_outcome: HashMap<&str, i32> = HashMap::from([
        ("A X", 3), // 0 for lose, 3 for scissors
        ("A Y", 4), // 3 for draw, 1 for rock
        ("A Z", 8), // 6 for win, 2 for paper
        ("B X", 1), // 0 for lose, 1 for rock
        ("B Y", 5), // 3 for draw, 2 for paper
        ("B Z", 9), // 6 for win, 3 for scissors
        ("C X", 2), // 0 for lose, 2 for paper
        ("C Y", 6), // 3 for draw, 3 for scissors
        ("C Z", 7), // 6 for win, 1 for rock
    ]);

    const INPUT_FILE: &str = "./input/day2.txt";
    let file = File::open(INPUT_FILE).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut sum_choice = 0;
    let mut sum_outcome = 0;
    for line in reader.lines() {
        let line = line.expect("Cannot parse line");

        match scores_choice.get::<str>(&line) {
            None => println!("Error, cannot match choice {}", &line),
            Some(score) => sum_choice += score,
        }

        match scores_outcome.get::<str>(&line) {
            None => println!("Error, cannot match outcome {}", &line),
            Some(score) => sum_outcome += score,
        }
    }

    println!("Strategy (choice) results in score: {}", sum_choice);
    println!("Strategy (outcome) results in score: {}", sum_outcome);

    println!("");
}
