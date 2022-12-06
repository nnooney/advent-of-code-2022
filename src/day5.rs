use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn day_five() {
    println!("Day 5:");

    const INPUT_FILE: &str = "./input/day5.txt";
    let file = File::open(INPUT_FILE).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut initial_state: Vec<String> = Vec::new();
    let mut stacks_9000: Vec<Vec<char>> = Vec::new();
    let mut stacks_9001: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Cannot parse line");

        if line.is_empty() {
            stacks_9000 = build_state(&initial_state);
            stacks_9001 = build_state(&initial_state);
        } else if stacks_9000.len() == 0 {
            initial_state.push(line);
        } else {
            process_move_9000(&mut stacks_9000, &line);
            process_move_9001(&mut stacks_9001, &line);
        }
    }
    print_top_stack(&stacks_9000);
    print_top_stack(&stacks_9001);

    println!("");
}

fn build_state(initial_state: &Vec<String>) -> Vec<Vec<char>> {
    let num_stacks = initial_state
        .last()
        .expect("vec is empty!")
        .split(' ')
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>()
        .len();

    let mut stacks = Vec::new();
    stacks.resize(num_stacks, Vec::new());

    for layer in initial_state[..initial_state.len() - 1].iter().rev() {
        for i in 0..num_stacks {
            let item = layer
                .chars()
                .nth(i * 4 + 1)
                .expect("Layer {i} is too short");
            if item != ' ' {
                stacks[i].push(item);
            }
        }
    }

    return stacks;
}

fn process_move_9000(stacks: &mut Vec<Vec<char>>, command: &str) {
    let command_parts = command.split(' ').collect::<Vec<&str>>();

    let amount: u32 = command_parts[1].parse().expect("Amount not a number");
    let from: usize = command_parts[3].parse().expect("From not a number");
    let to: usize = command_parts[5].parse().expect("To not a number");

    for _ in 0..amount {
        let item = stacks
            .get_mut(from - 1)
            .expect("From too big")
            .pop()
            .expect("stack is empty");
        stacks.get_mut(to - 1).expect("To too big").push(item);
    }
}

fn process_move_9001(stacks: &mut Vec<Vec<char>>, command: &str) {
    let command_parts = command.split(' ').collect::<Vec<&str>>();

    let amount: u32 = command_parts[1].parse().expect("Amount not a number");
    let from: usize = command_parts[3].parse().expect("From not a number");
    let to: usize = command_parts[5].parse().expect("To not a number");

    let mut temp_stack: Vec<char> = Vec::new();
    for _ in 0..amount {
        temp_stack.push(
            stacks
                .get_mut(from - 1)
                .expect("From too big")
                .pop()
                .expect("stack is empty"),
        );
    }
    for _ in 0..amount {
        stacks
            .get_mut(to - 1)
            .expect("To too big")
            .push(temp_stack.pop().expect("stack is empty"));
    }
}

fn print_top_stack(stacks: &Vec<Vec<char>>) {
    for stack in stacks {
        print!("{}", stack.last().expect("stack is empty"));
    }
    println!("");
}
