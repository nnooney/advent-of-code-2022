use std::collections::HashMap;
use std::fs::File;
use std::i32;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

pub fn day_seven() {
    println!("Day 7:");

    const INPUT_FILE: &str = "./input/day7.txt";
    let file = File::open(INPUT_FILE).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut directory_contents: HashMap<String, Vec<FsEntry>> =
        HashMap::from([("/".to_string(), Vec::new())]);
    let mut pwd = PathBuf::new();
    for line in reader.lines() {
        let line = line.expect("Cannot parse line");

        if line.starts_with("$") {
            let command_parts = line.split(" ").collect::<Vec<&str>>();
            match command_parts[1] {
                "cd" => match command_parts[2] {
                    "/" => {
                        pwd.clear();
                        pwd.push("/");
                    }
                    ".." => {
                        pwd.pop();
                    }
                    path => {
                        pwd.push(path);
                    }
                },
                "ls" => (),
                other => println!("Unhandled command: {}", other),
            }
        } else {
            let listing_parts = line.split(" ").collect::<Vec<&str>>();
            let pwd_path = pwd.to_str().unwrap();
            match listing_parts[0] {
                "dir" => {
                    let abs_path = match pwd_path {
                        "/" => format!("/{}", listing_parts[1]),
                        noroot => format!("{}/{}", noroot, listing_parts[1]),
                    };

                    // Add directory entry to pwd
                    directory_contents
                        .get_mut(pwd_path)
                        .expect(&format!("No directory: {}", pwd_path))
                        .push(FsEntry::Directory(abs_path.clone()));

                    // Add new directory to HashMap
                    directory_contents.insert(abs_path, Vec::new());
                }
                size => {
                    let size = size.parse::<i32>().unwrap();

                    directory_contents
                        .get_mut(pwd_path)
                        .expect(&format!("No directory: {}", pwd_path))
                        .push(FsEntry::File(listing_parts[1].to_string(), size));
                }
            }
        }
    }

    let mut directory_sizes: HashMap<String, i32> = HashMap::new();
    while !directory_sizes.contains_key("/") {
        for (name, contents) in &directory_contents {
            match compute_directory_size(&contents, &directory_sizes) {
                -1 => {}
                size => {
                    directory_sizes.insert(name.clone(), size);
                }
            }
        }
    }

    let mut sum_small_dirs = 0;
    for (_name, size) in &directory_sizes {
        if size <= &100000 {
            sum_small_dirs += size;
        }
    }

    println!("Sum(size) of dirs with size <= 100000: {}", sum_small_dirs);

    let bytes_to_free = directory_sizes.get("/").unwrap() - 40000000;
    println!("Need to free {} bytes", bytes_to_free);

    let mut dir_to_delete: (&String, &i32) = (&"".to_string(), &i32::MAX);
    for dir in &directory_sizes {
        if dir.1 >= &bytes_to_free && dir.1 < &dir_to_delete.1 {
            dir_to_delete = dir;
        }
    }
    println!(
        "Remove directory {} (size: {}) to process update",
        dir_to_delete.0, dir_to_delete.1
    );

    println!("");
}

fn compute_directory_size(contents: &Vec<FsEntry>, directory_sizes: &HashMap<String, i32>) -> i32 {
    let mut total_size: i32 = 0;
    for item in contents {
        match item {
            FsEntry::File(_, size) => {
                total_size += size;
            }
            FsEntry::Directory(name) => match directory_sizes.get(name) {
                None => return -1,
                Some(size) => total_size += size,
            },
        }
    }

    return total_size;
}

#[derive(Debug)]
enum FsEntry {
    File(String, i32),
    Directory(String),
}
