
use std::{convert::TryInto, collections::HashMap};

use aoc_util::{self, get_delimited_input};


fn main() {
    part1();
    part2();
}

fn part1() {

    let commands = get_delimited_input("10", " ");

    let interest_cycles = vec![20,60,100,140,180,220];

    let cycle_vals = get_cycle_values(&commands);

    let mut interest_sum = 0;
    for c in &interest_cycles {
        interest_sum += c * cycle_vals.get(c).unwrap();
    }

    println!("Part 1: {}", interest_sum);
}

fn part2() {
    let commands = get_delimited_input("10", " ");
    let mut screen = vec![vec!["."; 40]; 6];

    let cycle_vals = get_cycle_values(&commands);

    for cycle in 1..=240 as i32 {
        let r: usize = (((cycle - 1) / 40)).try_into().unwrap();
        let c: usize = (((cycle - 1) % 40)).try_into().unwrap();

        let cursor: i32 = c.try_into().unwrap();
        if should_print(cursor, *cycle_vals.get(&cycle).unwrap()) {
            screen[r][c] = "\u{2588}";
        }
    }

    println!("Part 2:");
    print_screen(&screen);
}

fn should_print(cursor: i32, x: i32) -> bool {
    cursor >= (x-1) && cursor <= (x+1)
}

fn print_screen(screen: &Vec<Vec<&str>>) {
    for row in screen {
        println!("{}", row.join(""));
    }
}

fn get_cycle_values(commands: &Vec<Vec<String>>) -> HashMap<i32, i32> {
    let mut cycle = 1;
    let mut x = 1;

    let mut cycle_vals = HashMap::new();

    for command in commands {
        match command[0].as_str() {
            "noop" => {
                cycle_vals.insert(cycle, x);
                cycle += 1;
            }
            _ => {
                let v = command[1].parse::<i32>().unwrap();
                let next_cycle = cycle + 1;
                cycle_vals.insert(cycle, x);
                cycle_vals.insert(next_cycle, x);
                cycle += 2;
                x += v;
            }
        }
    }

    cycle_vals
}