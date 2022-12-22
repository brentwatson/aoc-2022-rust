use crate::reader::read_aoc_file;

pub fn day1() {
    if let Ok(lines) = read_aoc_file(1) {
        let mut count = 0;
        let mut totals = Vec::new();

        for line in lines {
            if let Ok(data) = line {
                if data == "" {
                    totals.push(count);
                    count = 0;
                } else {
                    count += data.parse::<i32>().unwrap();
                }
            }
        }
        // part 1
        println!("Most Calories: {:?}", totals.iter().max());
        // part 2
        totals.sort();
        let top_3 = totals.iter().rev().take(3).sum::<i32>();
        println!("Top 3 sum: {}", top_3);
    }
}