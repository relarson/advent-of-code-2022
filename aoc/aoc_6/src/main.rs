use aoc_util;
use aoc_util::get_input;

use std::collections::HashSet;


fn main() {
    part1();
    part2();
}


fn part1() {
    let datastream = &get_input("6")[0];
    println!("Part 1: {}", find_marker(datastream, 4));
}

fn part2() {
    let datastream = &get_input("6")[0];
    println!("Part 2: {}", find_marker(datastream, 14));
}

fn find_marker(datastream: &str, length: usize) -> usize {
    let mut marker = length;

    let mut seen = HashSet::new();

    for window in datastream.chars().collect::<Vec<char>>().windows(length) {
        let mut start = true;
        for chr in window {
            if seen.contains(chr) {
                seen.clear();
                marker += 1;
                start = false;
                break;
            } else {
                seen.insert(chr);
            }
        }
        if start {
            return marker;
        }
    }
    marker
}