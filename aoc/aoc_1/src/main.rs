use aoc_util;
use aoc_util::get_grouped_numeric_input;

fn main() {
    part1();
    part2();
}

fn part1() {
    let groups : Vec<Vec<u32>> = get_grouped_numeric_input("1");

    let most = sort_calories(groups)[0];

    println!("Part 1: {}", most);
}

fn part2() {
    let groups : Vec<Vec<u32>> = get_grouped_numeric_input("1");

    let calories = sort_calories(groups);

    let top3 = calories[0] + calories[1] + calories[2];

    println!("Part 2: {}", top3);
}

fn sort_calories(groups: Vec<Vec<u32>>) -> Vec<u32> {

    let mut calorie_counts = Vec::<u32>::new();

    for group in &groups {
        let mut calories = 0;
        for food in group {
            calories += food;
        }
        calorie_counts.push(calories)
    }

    calorie_counts.sort();
    calorie_counts.reverse();

    calorie_counts
}
