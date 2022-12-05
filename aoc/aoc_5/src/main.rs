use aoc_util;
use aoc_util::get_delimited_input;


fn main() {
    part1();
    part2();
}

/** top of input
    [M]             [Z]     [V]    
    [Z]     [P]     [L]     [Z] [J]
[S] [D]     [W]     [W]     [H] [Q]
[P] [V] [N] [D]     [P]     [C] [V]
[H] [B] [J] [V] [B] [M]     [N] [P]
[V] [F] [L] [Z] [C] [S] [P] [S] [G]
[F] [J] [M] [G] [R] [R] [H] [R] [L]
[G] [G] [G] [N] [V] [V] [T] [Q] [F]
 1   2   3   4   5   6   7   8   9 

 */

fn part1() {
    println!("Part 1: {}", move_crates(true));
}

fn part2() {
    println!("Part 2: {}", move_crates(false));
}

fn move_crates(one_at_a_time: bool) -> String {
    let mut stacks = get_starting_stacks();
    let moves : Vec<Vec<String>> = get_delimited_input("5", " ");

    for mve in &moves {
        let count = mve[1].parse::<usize>().unwrap();
        let src = mve[3].parse::<usize>().unwrap() - 1; //my stacks are 0 indexed
        let dest = mve[5].parse::<usize>().unwrap() - 1; //my stacks are 0 indexed

        let idx = stacks[src].len() - count;

        let mut moving = stacks[src].drain(idx..).collect::<Vec<char>>();
        if one_at_a_time {
            moving.reverse();
        }

        stacks[dest].append(&mut moving);
    }

    let mut answer = String::from("");

    for stack in &stacks {
        answer.push(stack[stack.len() - 1])
    }

    answer
}

fn get_starting_stacks() -> Vec<Vec<char>> {
    let mut stacks = Vec::new();

    stacks.push("GFVHPS".chars().collect());
    stacks.push("GJFBVDZM".chars().collect());
    stacks.push("GMLJN".chars().collect());
    stacks.push("NGZVDWP".chars().collect());
    stacks.push("NRCB".chars().collect());
    stacks.push("VRSMPWLZ".chars().collect());
    stacks.push("THP".chars().collect());
    stacks.push("QRSNCHZV".chars().collect());
    stacks.push("FLGPVQJ".chars().collect());

    stacks
}



