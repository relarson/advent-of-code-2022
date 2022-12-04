//use curl::easy::Easy;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::iter::Product;
use std::ops::Add;
use std::str::FromStr;
use std::fmt::Debug;

pub fn get_input(day: &str) -> Vec<String> {
    let f = File::open("./aoc_util/input_files/day_".to_string() + day)
        .expect(&format!("Please create an input file for day {}", day));
    let br = BufReader::new(f);

    let mut lines = Vec::new();

    for line_res in br.lines() {
        match line_res {
            Ok(line) => lines.push(line),
            Err(e) => panic!("{}", e)
        }
    }

    lines
}

pub fn get_delimited_input(day: &str, delimiter: &str) -> Vec<Vec<String>> {
    let lines = get_input(day);

    let mut output = Vec::new();
    for line in &lines {
        output.push(line.split(delimiter).map(String::from).collect())
    }

    output
}


pub fn get_numeric_input<T>(day: &str) -> Vec<T>
    where T: Add + Product + FromStr,
          <T as FromStr>::Err: Debug
{
    let input = get_input(day);
    let mut nums = Vec::new();

    for line in &input {
        let n = line.parse::<T>().unwrap();
        nums.push(n);
    }

    nums
}

pub fn get_grouped_numeric_input<T>(day: &str) -> Vec<Vec<T>>
    where T: Add + Product + FromStr,
        <T as FromStr>::Err: Debug
{
    let input = get_input(day);
    let mut groups = Vec::new();

    let mut current = Vec::new();

    for line in &input {
        if line.is_empty() {
            groups.push(current);
            current = Vec::new();
        } else {
            let n = line.parse::<T>().unwrap();
            current.push(n);
        }
    }

    groups
}
