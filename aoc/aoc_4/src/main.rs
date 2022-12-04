use aoc_util;
use aoc_util::get_delimited_input;


fn main() {
    part1();
    part2();
}

fn part1() {
    let paired_assignments : Vec<Vec<String>> = get_delimited_input("4", ",");

    let mut fully_encompassed = 0;

    for pair in &paired_assignments {
        let (f_s, f_e) = parse_assignment(&pair[0]);
        let (s_s, s_e) = parse_assignment(&pair[1]);

        if (f_s >= s_s && f_e <= s_e) || (s_s >= f_s && s_e <= f_e) {
            fully_encompassed += 1;
        }
    }

    println!("Part 1: {}", fully_encompassed);
}

fn part2() {

    let paired_assignments : Vec<Vec<String>> = get_delimited_input("4", ",");

    let mut has_overlap = 0;

    for pair in &paired_assignments {
        let (f_s, f_e) = parse_assignment(&pair[0]);
        let (s_s, s_e) = parse_assignment(&pair[1]);

        if f_e < s_s || s_e < f_s {
            continue;
        } else {
            has_overlap += 1
        }
    }

    println!("Part 2: {}", has_overlap);
}

fn parse_assignment(input: &str) -> (u32, u32) {
    let (start, end) = input.split_once("-").unwrap();

    return (start.parse::<u32>().unwrap(), end.parse::<u32>().unwrap());
}


