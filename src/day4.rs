use std::ops::RangeInclusive;
use crate::reader::read_aoc_file;

pub fn day4() {
    // Part 1
    let lines = read_aoc_file(4).unwrap();
    let lines_overlapping = lines.filter(| line| is_overlapping_fully(line.as_ref().unwrap()));
    println!("{}", lines_overlapping.count());

    // Part 2
    let lines = read_aoc_file(4).unwrap();
    let lines_overlapping = lines.filter(| line| is_overlapping_at_all(line.as_ref().unwrap()));
    println!("{}", lines_overlapping.count());

}

fn is_overlapping_fully(data: &String) -> bool {
    let (first_range, second_range) = grab_ranges(data);

    let contains_all = first_range.clone().all(|x| second_range.contains(&x)) ||
        second_range.clone().all(|x| first_range.contains(&x));

    contains_all
}

fn is_overlapping_at_all(data: &String) -> bool {
    let (first_range, second_range) = grab_ranges(data);

    let contains_all = first_range.clone().any(|x| second_range.contains(&x)) ||
        second_range.clone().any(|x| first_range.contains(&x));

    contains_all
}

fn grab_ranges(data: &String) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let split = data.split(",");
    let vec: Vec<&str> = split.collect();
    let first: &str = vec[0];
    let second: &str = vec[1];

    let first_set: Vec<&str> = first.split("-").collect();
    let second_set: Vec<&str> = second.split("-").collect();

    let first_range = (first_set[0].parse::<i32>().unwrap())..=(first_set[1].parse::<i32>().unwrap());
    let second_range = (second_set[0].parse::<i32>().unwrap())..=(second_set[1].parse::<i32>().unwrap());
    (first_range, second_range)
}