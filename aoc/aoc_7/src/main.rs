use std::collections::HashMap;

use aoc_util;
use aoc_util::get_input;


fn main() {
    part1();
    part2();
}


fn part1() {
    let input = get_input("7");

    let dir_sizes = get_dir_sizes(&input);

    let mut total_size_of_small = 0;

    for (_, size) in dir_sizes {
        if size <= 100000 {
            total_size_of_small += size;
        }
    }

    println!("Part 1: {}", total_size_of_small);
}

fn part2() {
    let input = get_input("7");

    let dir_sizes = get_dir_sizes(&input);

    let root_size = dir_sizes.get(&String::from("/")).unwrap();
    let free_space = 70000000 - root_size;
    let needed_space = 30000000 - free_space;

    let mut size_of_smallest_dir_to_delete = *root_size; // worst case is root

    for (_, size) in dir_sizes {
        if size >= needed_space && size < size_of_smallest_dir_to_delete {
            size_of_smallest_dir_to_delete = size;
        }
    }

    println!("Part 2: {}", size_of_smallest_dir_to_delete);
}

fn get_dir_sizes(commands: &Vec<String>) -> HashMap<String, u64> {
    let mut dir_sizes = HashMap::new();
    let mut cwd_stack = Vec::new();
    for command in commands {
        let parts: Vec<&str> = command.split(" ").collect();
        let mut cwd = String::from(cwd_stack.last().unwrap_or(&String::from("")));
        if cwd != "/" && cwd != "" {
            cwd += "/";
        }
        match parts[0] {
            "$" => {
                match parts[1] {
                    "cd" => match parts[2] {
                        ".." => {
                            cwd_stack.pop();
                        }
                        "/" => {
                            cwd_stack.clear();
                            cwd_stack.push(String::from("/"));
                        }
                        _ => {
                            let new_cwd = cwd + &String::from(parts[2]);
                            cwd_stack.push(new_cwd);
                        }
                    },
                    _ => continue // ignore ls lines
                }
            },
            "dir" => continue, // ignore dirs, we account for them in cd and via stack
            _ => { // this means we are looking at a file, at its size to all dirs in cwd_stack
                let bytes = parts[0].parse::<u64>().unwrap();
                for dir in &cwd_stack {
                    *dir_sizes.entry(dir.to_string()).or_insert(0) += bytes;
                }
            }
        }
    }

    dir_sizes
}
