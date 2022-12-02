use aoc_util;
use aoc_util::get_space_delimited_input;

fn main() {
    part1();
    part2();
}

fn part1() {
    let games : Vec<Vec<String>> = get_space_delimited_input("2");

    let mut score = 0;

    for game in games {
        score += score_game(&game[0], &game[1])
    }

    println!("Part 1: {}", score);
}

fn part2() {
    let games : Vec<Vec<String>> = get_space_delimited_input("2");

    let mut score = 0;

    for game in games {
        score += score_game_2(&game[0], &game[1])
    }

    println!("Part 2: {}", score);
}

fn score_game_2(opp: &str, result: &str) -> u32 {
    match opp {
        "A" => match result {
            "X" => 3, // lose: 0 + scissors: 3 
            "Y" => 4, // tie: 3 + rock: 1, 
            "Z" => 8, // win: 6 + paper: 2,
            _ => panic!("Invalid input: {}", result)
        },
        "B" => match result {
            "X" => 1, // lose: 0 + rock: 1,
            "Y" => 5, // tie: 3 + paper: 2,
            "Z" => 9, // win: 6 + scissors: 3
            _ => panic!("Invalid input: {}", result)
        },
        "C" => match result {
            "X" => 2, // lose: 0 + paper: 2,
            "Y" => 6, // tie: 3 + scissors: 3
            "Z" => 7, // win: 6 + rock: 1,
            _ => panic!("Invalid input: {}", result)
        },
        _ => panic!("Invalid input: {}", opp)
    }
}

fn score_game(opp: &str, me: &str) -> u32 {
    match opp {
        "A" => match me {
            "X" => 4, // tie: 3 + rock: 1,
            "Y" => 8, // win: 6 + paper: 2,
            "Z" => 3, // lose: 0 + scissors: 3
            _ => panic!("Invalid input: {}", me)
        },
        "B" => match me {
            "X" => 1, // lose: 0 + rock: 1,
            "Y" => 5, // tie: 3 + paper: 2,
            "Z" => 9, // win: 6 + scissors: 3
            _ => panic!("Invalid input: {}", me)
        },
        "C" => match me {
            "X" => 7, // win: 6 + rock: 1,
            "Y" => 2, // lose: 0 + paper: 2,
            "Z" => 6, // tie: 3 + scissors: 3
            _ => panic!("Invalid input: {}", me)
        },
        _ => panic!("Invalid input: {}", opp)
    }
}

