
use std::collections::HashMap;

use aoc_util::{self, get_delimited_input};


fn main() {
    part1();
    part2();
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    fn up(&self) -> Point {
        return Point{ x: self.x, y: self.y + 1};
    }

    fn down(&self) -> Point {
        return Point{ x: self.x, y: self.y - 1};
    }

    fn left(&self) -> Point {
        return Point{ x: self.x - 1, y: self.y};
    }

    fn right(&self) -> Point {
        return Point{ x: self.x + 1, y: self.y};
    }

    fn diagonal(&self, delta_x: i32, delta_y: i32) -> Point {
        return Point{x: self.x + delta_x, y: self.y + delta_y}
    }

    fn move_towards(&self, other: Point) -> Point {
        if self.touches(other) {
            // short circuit, no move needed
            return *self;
        }
        let delta_x = other.x - self.x;
        let delta_y = other.y - self.y;
        if delta_x == 0 {
            if delta_y > 0 {
                return self.up();
            } else {
                return self.down();
            }
        } else if delta_y == 0 {
            if delta_x > 0 {
                return self.right();
            } else {
                return self.left();
            }
        } else {
            // other is away in both directions
            // we know delta_x & delta_y arent both 1 because then touches
            // would have returned true and triggered our short circuit
            let move_x = if delta_x > 0 { 1 } else { -1 };
            let move_y = if delta_y > 0 { 1 } else { -1 };
            return self.diagonal(move_x, move_y);
        }
    }

    fn touches(&self, other: Point) -> bool {
        return self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }
}


#[derive(PartialEq, Debug, Clone, Copy)]
enum Dir {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
    None
}


fn part1() {
    let directions = get_delimited_input("9", " ");

    let moves: Vec<Dir> = directions.iter()
        .map(|d| parse_move(d))
        .collect();

    let mut tail_visits: HashMap<Point, u32> = HashMap::new();

    let mut knots = vec![Point {x: 0, y:0}; 2];

    *tail_visits.entry(knots[knots.len() - 1]).or_insert(0) += 1;

    for dir in moves {
        knots = handle_move(knots, dir, &mut tail_visits);
    }

    let mut total_visited = 0;
    for (_, count) in tail_visits {
        if count > 0 {
            total_visited += 1;
        }
    }

    println!("Part 1: {}", total_visited);
}

fn part2() {
    let directions = get_delimited_input("9", " ");

    let moves: Vec<Dir> = directions.iter()
        .map(|d| parse_move(d))
        .collect();

    let mut tail_visits: HashMap<Point, u32> = HashMap::new();

    let mut knots = vec![Point {x: 0, y:0}; 10];

    *tail_visits.entry(knots[knots.len() - 1]).or_insert(0) += 1;

    for dir in moves {
        knots = handle_move(knots, dir, &mut tail_visits);
    }

    let mut total_visited = 0;
    for (_, count) in tail_visits {
        if count > 0 {
            total_visited += 1;
        }
    }

    println!("Part 2: {}", total_visited);
}

fn handle_move(knots: Vec<Point>, dir: Dir, tail_visits: &mut HashMap<Point, u32>) -> Vec<Point> {
    let mut new_knots = knots.clone();
    let mut direction = dir;

    while direction != Dir::None {
        let old_tail = new_knots[new_knots.len() - 1].clone();
        (new_knots, direction) = handle_move_step(new_knots, direction);
        if new_knots[new_knots.len() - 1] != old_tail {
            *tail_visits.entry(new_knots[new_knots.len() - 1]).or_insert(0) += 1;
        }
    }

    new_knots
}

fn handle_move_step(knots: Vec<Point>, dir: Dir) -> (Vec<Point>, Dir) {
    let mut moved_knots = knots.clone();
    let mut new_dir = Dir::None;
    match dir {
        Dir::Up(distance) => {
            moved_knots[0] = moved_knots[0].up();
            if distance > 1 { 
                new_dir = Dir::Up(distance - 1);
            }
        },
        Dir::Down(distance) => {
            moved_knots[0] = moved_knots[0].down();
            if distance > 1 { 
                new_dir = Dir::Down(distance - 1);
            }
        },
        Dir::Left(distance) => {
            moved_knots[0] = moved_knots[0].left();
            if distance > 1 { 
                new_dir = Dir::Left(distance - 1);
            }
        },
        Dir::Right(distance) => {
            moved_knots[0] = moved_knots[0].right();
            if distance > 1 { 
                new_dir = Dir::Right(distance - 1);
            }
        },
        Dir::None => panic!("Invalid move step None!")
    }
    for i in 1..moved_knots.len() {
        moved_knots[i] = moved_knots[i].move_towards(moved_knots[i-1]);
    }
    (moved_knots, new_dir)
}

fn parse_move(input: &Vec<String>) -> Dir {
    let distance = input[1].parse().unwrap();
    match input[0].as_str() {
        "U" => Dir::Up(distance),
        "D" => Dir::Down(distance),
        "L" => Dir::Left(distance),
        "R" => Dir::Right(distance),
        _ => panic!("Invalid input: {}", input.join(" "))
    }
}


