use aoc_util;
use aoc_util::get_input;


fn main() {
    part1();
    part2();
}

fn part1() {
    let packs : Vec<String> = get_input("3");

    let mut score = 0;

    for pack in packs {
        let (first, second) = pack.as_str().split_at(pack.len() / 2);
        let common = find_common_item(first, second);

        if common.is_ascii_lowercase() {
            score += (common as u32) - 96;
        } else if common.is_ascii_uppercase() {
            score += (common as u32) - 64 + 26;
        }
    }

    println!("Part 1: {}", score);
}

fn part2() {

    let packs : Vec<String> = get_input("3");

    let mut score = 0;

    for n in (0..packs.len() - 2).step_by(3) {
        let badge = find_badge(&packs[n].as_str(), &packs[n+1].as_str(), &packs[n+2].as_str());
        println!("{}: {}", n,  badge);
        if badge.is_ascii_lowercase() {
            score += (badge as u32) - 96;
        } else if badge.is_ascii_uppercase() {
            score += (badge as u32) - 64 + 26;
        }
    }

    println!("Part 2: {}", score);
}

fn find_common_item(first: &str, second: &str) -> char {
    for chr in first.chars() {
        if second.contains(chr) {
            return chr
        }
    }
    panic!("no common item found!")
}

fn find_badge(first: &str, second: &str, third: &str) -> char {
    for chr in first.chars() {
        if second.contains(chr) && third.contains(chr) {
            return chr
        }
    }
    panic!("no common item found!")
}

