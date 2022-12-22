use std::fs::File;
use std::io::{BufReader, Lines};
use if_chain::if_chain;
use crate::reader::read_aoc_file;

pub fn day8() {
    if let Ok(lines) = read_aoc_file(8) {
        let trees = extract_tree_grid(lines);
        let rows = 99;
        let cols = 99;

        let mut count = 0;
        for x1 in 0..rows.clone() {
            'next_tree: for y1 in 0..cols.clone() {
                //if x1 != 2 || y1 != 1 { continue 'next_tree; }

                let v = &trees[x1][y1];
                //println!("checking: {},{} [{}]", x1, y1, v);

                if x1 == 0 || y1 == 0 || x1 == rows - 1 || y1 == cols - 1 {
                    //edge tree
                    count += 1;
                    continue 'next_tree;
                }

                let mut all_top_visible = true;
                //println!("{:?}", 0..x1.wrapping_sub(0));
                for x in 0..(x1.wrapping_sub(0)) {
                    if !check_if_visible(trees.clone(), v, x.clone(), y1.clone()) {
                        all_top_visible = false;
                    }
                }

                let mut all_bottom_visible = true;
                for x in (x1+1)..rows {
                    if !check_if_visible(trees.clone(), v, x, y1) {
                        all_bottom_visible = false;
                    }
                }

                let mut all_left_visible = true;
                for y in 0..y1.wrapping_sub(0) {
                    if !check_if_visible(trees.clone(), v, x1, y) {
                        all_left_visible = false;
                    }
                }

                let mut all_right_visible = true;
                for y in (y1+1)..cols {
                    if !check_if_visible(trees.clone(), v, x1, y) {
                        all_right_visible = false;
                    }
                }

                if all_top_visible || all_bottom_visible || all_left_visible || all_right_visible {
                    count += 1;
                }

                //
                // check_bottom()
                // check_left()
                // check_right()


                //

                //
                // let mut all_y_visible = true;
                // for y in 0..cols.clone() {
                //     if y1 == y { break; }
                //     if !check_if_visible(trees.clone(), v, x1, y) {
                //         all_x_visible = false;
                //     }
                // }
                //
                // if all_x_visible || all_y_visible {
                //     println!("Visible!");
                //     count += 1;
                // } else {
                //     println!("Nope!")
                // }

                //
                // 'outer: for x in 0..rows {
                //     for y in 0..rows {
                //
                //         if check_if_any_taller() {
                //             count += 1;
                //         }
                //     }
                // }
            }
        }

        println!("Visible Trees: {}", count);
    }
}

fn check_if_visible(trees: Vec<Vec<i32>>, v: &i32, x: usize, y: usize) -> bool {
    // let top = try_get_value(trees.clone(), x, y.wrapping_add(1));
    // let bottom = try_get_value(trees.clone(), x, y.wrapping_sub(1));
    // let left = try_get_value(trees.clone(), x.wrapping_sub(1), y);
    // let right = try_get_value(trees.clone(), x.wrapping_add(1), y);

    //println!("checking against: {x},{y}: {:?}", try_get_value(trees.clone(), x, y));
    match try_get_value(trees, x, y) {
        Some(other) => v > &other,
        None => panic!("Yikes!"), // edge tree is always visible
    }


    // if_chain! {
    //     if let Some(t) = top;
    //     if let Some(b) = bottom;
    //     if let Some(l) = left;
    //     if let Some(r) = right;
    //     then {
    //         if t < *v && b < *v && l < *v && r < *v {
    //             println!("Found tall tree at {},{}: {}", x, y, v);
    //
    //         }
    //     } else {
    //
    //         println!("Found tree at edge {},{}: {}", x, y, v);
    //     }
    // }
}

fn try_get_value(trees: Vec<Vec<i32>>, x: usize, y: usize) -> Option<i32> {
    if let Some(row) = &trees.get(x) {
        if let Some(col) = row.get(y) {
            return Some(col.clone());
        }
    }
    None
}

fn extract_tree_grid(lines: Lines<BufReader<File>>) -> Vec<Vec<i32>> {
    let mut trees: Vec<Vec<i32>> = vec![];
    let mut index = 0;
    for line in lines {
        if let Ok(data) = line {
            if !data.is_empty() {
                trees.push(vec![]);
                for c in data.chars().into_iter() {
                    let row = trees.get_mut(index).unwrap();
                    row.push(c.to_string().parse::<i32>().unwrap());
                }
            }
        }
        index += 1;
    }
    trees
}