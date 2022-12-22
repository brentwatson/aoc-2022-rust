#![feature(take)]
#![feature(slice_take)]

use std::collections::HashSet;
use rand::Fill;
use crate::reader::read_aoc_file;

pub fn day6() {
    let line = read_aoc_file(6).unwrap().next().unwrap().unwrap();
    println!("Line: {}", line);
    let data: Vec<char> = line.chars().collect();

    // Part 1
    for i in 0..line.len() - 3 {
        let chunk= data[i..i+4].iter();

        let mut hs = HashSet::new();
        if chunk.clone().all(move |x| hs.insert(x)) {
            println!("Index: {} - [{:?}]", i+4, chunk.collect::<String>());
            break;
        }
    }

    // Part 2
    for i in 0..line.len() - 15 {
        let chunk= data[i..i+14].iter();

        let mut hs = HashSet::new();
        if chunk.clone().all(move |x| hs.insert(x)) {
            println!("Index: {} - [{:?}]", i+14, chunk.collect::<String>());
            break;
        }
    }
}