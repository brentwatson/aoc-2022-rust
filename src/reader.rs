use std;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub fn read_aoc_file(day: i32) -> std::io::Result<Lines<BufReader<File>>> {
    let input_file = format!("/Users/bwats2/code/aoc-2022-rust/input/day{}.txt", day);

    let file = File::open(input_file)?;
    Ok(BufReader::new(file).lines())
}
