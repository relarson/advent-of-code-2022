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
            Err(e) => panic!(e)
        }
    }

    lines
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

/*fn get_input_from_web(day: &str) -> String {
    let mut bytes = Vec::new();
    let mut easy = Easy::new();
    let url = "https://adventofcode.com/2020/day/".to_string() + day + "/input";
    easy.url(url.as_str()).unwrap();

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            bytes.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    let s = String::from_utf8(bytes).expect("Found invalid UTF-8");
    s
}*/
