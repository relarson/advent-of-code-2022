
use aoc_util::{self, get_int_matrix};


fn main() {
    part1();
    part2();
}


fn part1() {
    let matrix = get_int_matrix("8");

    let mut visible = Vec::new();

    for r in 0..matrix.len() {
        visible.push(vec![false; matrix[r].len()]);
    }


    for r in 0..matrix.len() {
        let row = &matrix[r];
        // first, check going forwards
        // 0 is a valid tree height that can be visible
        let mut blocked_height = &row[0];
        visible[r][0] = true;

        for c in 0..row.len() {
            let tree = &row[c];
            if tree > blocked_height {
                blocked_height = tree;
                visible[r][c] = true;
            }
        }

        // next, check going backwards
        // 0 is a valid tree height that can be visible
        let last_col = row.len() - 1;
        blocked_height = &row[last_col];
        visible[r][last_col] = true;

        for c in (0..row.len()).rev() {
            let tree = &row[c];
            if tree > blocked_height {
                blocked_height = tree;
                visible[r][c] = true;
            }
        }
    }

    // then check from above and below
    let cols = matrix[0].len();
    for c in 0..cols {
        let mut blocked_height = &matrix[0][c];
        visible[0][c] = true;
        for r in 0..matrix.len() {
            let tree = &matrix[r][c];
            if tree > blocked_height {
                blocked_height = tree;
                visible[r][c] = true;
            }
        }

        let last_row = matrix.len() - 1;
        blocked_height = &matrix[last_row][c];
        visible[last_row][c] = true;
        for r in (0..matrix.len()).rev() {
            let tree = &matrix[r][c];
            if tree > blocked_height {
                println!("{} is taller than {}", tree, blocked_height);
                blocked_height = tree;
                visible[r][c] = true;
            } else {
                println!("{} is NOT taller than {}", tree, blocked_height);
            }
        }
    }

/*     for row in &visible {
        let mapped: Vec<&str> = row.iter().map(|b| if *b { "V" } else { "." }).collect();
        println!("{}", mapped.join(" "));
    } */

    let visible_count = visible.iter()
        .flat_map(|r| {
            r.iter()
                .map(|b| if *b { 1 } else { 0 })
        })
        .reduce(|count, item| {
            return count + item;
        })
        .unwrap();

    println!("Part 1: {}", visible_count);
}

fn part2() {


    let forest = get_int_matrix("8");

    let mut best = 0;

    for row in 0..forest.len() {
        for col in 0..(forest[row].len()) {
            let score = calc_score(row, col, &forest);
            if score > best {
                best = score;
            }
        }
    }

    println!("Part 2: {}", best);
}

fn calc_score(row: usize, col: usize, forest: &Vec<Vec<u32>>) -> u32 {
    let rows = forest.len();
    let cols = forest[0].len();

    // first and last row have 1 dir of 0
    if (row == 0) || (row == rows - 1) {
        return 0;
    }

    // first and last col have 1 dir of 0
    if (col == 0) || (col == cols - 1) {
        return 0;
    }

    let h = forest[row][col];

    let mut vis_south: u32 = 0;
    for r in (row+1)..rows {
        vis_south += 1;
        if forest[r][col] >= h {
            // last tree we can see this dir
            break;
        }
    }

    let mut vis_north: u32 = 0;
    for r in (0..row).rev() {
        vis_north += 1;
        if forest[r][col] >= h {
            // last tree we can see this dir
            break;
        }
    }

    let mut vis_east: u32 = 0;
    for c in (col+1)..cols {
        vis_east += 1;
        if forest[row][c] >= h {
            // last tree we can see this dir
            break;
        }
    }

    let mut vis_west: u32 = 0;
    for c in (0..col).rev() {
        vis_west += 1;
        if forest[row][c] >= h {
            // last tree we can see this dir
            break;
        }
    }

    return vis_north * vis_east * vis_south * vis_west;
}
