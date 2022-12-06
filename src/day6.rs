use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn day_six() {
    println!("Day 6:");

    const INPUT_FILE: &str = "./input/day6.txt";
    let file = File::open(INPUT_FILE).expect("Cannot open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Cannot parse line");

        let sequence: Vec<char> = line.chars().collect();
        const START_OF_PACKET_LENGTH: usize = 4;

        for i in START_OF_PACKET_LENGTH.. {
            let set: HashSet<&char> = HashSet::from_iter(&sequence[i - START_OF_PACKET_LENGTH..i]);
            if set.len() == START_OF_PACKET_LENGTH {
                println!("{} characters processed before start-of-packet", i);
                break;
            }
        }

        const START_OF_MESSAGE_LENGTH: usize = 14;

        for i in START_OF_MESSAGE_LENGTH.. {
            let set: HashSet<&char> = HashSet::from_iter(&sequence[i - START_OF_MESSAGE_LENGTH..i]);
            if set.len() == START_OF_MESSAGE_LENGTH {
                println!("{} characters processed before start-of-message", i);
                break;
            }
        }
    }

    println!("");
}
