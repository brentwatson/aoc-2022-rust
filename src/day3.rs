use std::collections::HashSet;
use std::str::Chars;
use crate::reader::read_aoc_file;

pub fn day3() {
    day3_part1();
    day3_part2();
}

pub fn day3_part1() {
    if let Ok(lines) = read_aoc_file(3) {
        let mut total = 0;
        for line in lines {
            if let Ok(data) = line {
                let half = data.len() / 2;
                let first = data[..half].chars().into_iter();
                let last = data[half..].to_string();

                let char_match: char = intersecting_chars(first, last);

                total += calc_priority_value(char_match);
            }
        }
        println!("{}", total);
    }
}

pub fn day3_part2() {
    if let Ok(mut lines) = read_aoc_file(3) {
        let mut total = 0;

        loop {
            let line1 = lines.next();
            if line1.is_none() { break; }
            let line1 = line1.unwrap().unwrap();
            let line2 = lines.next().unwrap().unwrap();
            let line3 = lines.next().unwrap().unwrap();

            let mut chars1 = line1.chars();
            let mut chars2 = line2.chars();
            let mut chars3 = line3.chars();

            let intersection1: String = chars1.filter( | &c | line2.contains(c) ).collect();
            let intersection2  = chars3.find( | &c | intersection1.contains(c) ).unwrap();

            let value = calc_priority_value(intersection2);
            total += value;

        }
        println!("{}", total);
    }
}

fn intersecting_chars(first: Chars, second: String) -> char {
    first.to_owned().find(|&c|
        second.contains(c)
    ).unwrap()
}

// a through z have priorities 1 through 26.
// A through Z have priorities 27 through 52
fn calc_priority_value(char_match: char) -> u32{
    let ascii_val = char_match as u32;

    match char_match {
        'a'..='z' => ascii_val - 96,
        'A'..='Z' => ascii_val - 65 + 27,
        _ => panic!("Bad letter")
    }
}