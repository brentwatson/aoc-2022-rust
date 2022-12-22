use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufReader, Lines};
use crate::reader::read_aoc_file;

use regex::Regex;

pub fn day5() {
    day5_part_1();
    day5_part_2();
}

pub fn day5_part_1() {
    let mut lines = read_aoc_file(5).unwrap();
    let mut stacks = extract_stacks(&mut lines);

    lines.next(); // Skip blank line

    for line in lines {
        if let Ok(data) = line {
            println!("{}", data);
            let (num, from, to) = extract_move_data(data);

            for i in 0..num {
                stacks = move_items(stacks, from, to);
                println!("New Stacks: {:?}\n", stacks);
            }
        }
    }
    println!("FINAL STACK: {:?}", stacks);

    let answer = word_from_first_chars_in_stacks(stacks);
    println!("{}", answer);
}


pub fn day5_part_2() {
    let mut lines = read_aoc_file(5).unwrap();
    let mut stacks = extract_stacks(&mut lines);

    lines.next(); // Skip blank line

    for line in lines {
        if let Ok(data) = line {
            println!("{}", data);
            let (num, from, to) = extract_move_data(data);
            stacks = move_n_items(stacks, num, from, to);
            println!("New Stacks: {:?}\n", stacks);
        }
    }
    println!("FINAL STACK: {:?}", stacks);

    let answer = word_from_first_chars_in_stacks(stacks);
    println!("{}", answer);
}

fn word_from_first_chars_in_stacks(stacks: Vec<Vec<char>>) -> String {
    let first_chars = stacks.iter().map(|v|
        match v.first() {
            None => { ' ' }
            Some(x) => { x.clone() }
        }
    ).collect::<Vec<char>>();
    first_chars.iter().collect::<String>().trim().to_string()
}

fn move_items(stacks: Vec<Vec<char>>, from: usize, to: usize) -> Vec<Vec<char>> {
    println!("Move items from {:?} to {:?}", stacks.get(from), stacks.get(to));

    let mut stacks = stacks.clone();
    let from_stack = stacks.get(from).unwrap();
    let char = from_stack.first().unwrap().clone();

    let mut to_stack = stacks.get_mut(to).unwrap();
    to_stack.insert(0, char);

    let from_stack = stacks.get_mut(from).unwrap();
    from_stack.remove(0);

    stacks
}

fn move_n_items(stacks: Vec<Vec<char>>, num: usize, from: usize, to: usize) -> Vec<Vec<char>> {
    println!("Move {} items from {:?} to {:?}", num, stacks.get(from), stacks.get(to));

    let mut new_stacks = stacks.clone();
    let (first, _) = stacks.get(from).unwrap().split_at(num);

    let mut to_stack = new_stacks.get_mut(to).unwrap();
    for c in first.iter().rev() {
        to_stack.insert(0, c.clone());
    }

    let from_stack = new_stacks.get_mut(from).unwrap();
    for i in 0..num {
        from_stack.remove(0);
    }

    new_stacks
}

fn extract_stacks(lines: &mut Lines<BufReader<File>>) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for i in 0..9 {
        stacks.push(Vec::new());
    }

    let mut line = lines.next().unwrap().unwrap();
    while !line.starts_with(" 1") {
        println!("LINE: {}", line);
        let idx = line.len() / 4;
        let mut vec_index = 0;

        for idx in (1..line.len()).step_by(4) {
            let item = line.clone().chars().take(idx + 1).last().unwrap();
            let stack = &mut stacks[vec_index];
            if item != ' ' {
                stack.push(item);
            }
            vec_index += 1;
        }
        line = lines.next().unwrap().unwrap();
    }
    println!("Stacks: {:?}", stacks);
    stacks
}

fn extract_move_data(data: String) -> (usize, usize, usize) {
    let re = Regex::new(r"move (\d+) from (\d) to (\d)").unwrap();

    let items = re.captures(&*data).unwrap();
    let num = items.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let from = items.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let to = items.get(3).unwrap().as_str().parse::<usize>().unwrap();

    (
        num.to_string().parse::<usize>().unwrap(),
        from.to_string().parse::<usize>().unwrap() - 1,
        to.to_string().parse::<usize>().unwrap() - 1,
    )
}
