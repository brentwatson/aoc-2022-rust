use crate::day2::Outcome::{DRAW, LOSE, WIN};
use crate::day2::Selection::{PAPER, ROCK, SCISSORS};
use crate::reader::read_aoc_file;

pub fn day2() {
    day2_part1();
    day2_part2();
}

fn day2_part1() {
    if let Ok(lines) = read_aoc_file(2) {
        let mut total = 0;
        for line in lines {
            if let Ok(data) = line {
                let opponent = Selection::to_selection(data.chars().next().unwrap());
                let me = Selection::to_selection(data.chars().take(3).last().unwrap());

                let score = me.score(&opponent);
                total += score;
            }
        }
        println!("{}", total);
    }
}

fn day2_part2() {
    if let Ok(lines) = read_aoc_file(2) {
        let mut total = 0;
        for line in lines {
            if let Ok(data) = line {
                let opponent = Selection::to_selection(data.chars().next().unwrap());
                let outcome = Outcome::to_output(data.chars().take(3).last().unwrap());

                let selection = Selection::pick(outcome, &opponent);
                let score = selection.score(&opponent);
                total += score;
            }
        }
        println!("{}", total);
    }
}

enum Outcome {
    LOSE,
    DRAW,
    WIN,
}

impl Outcome {
    fn to_output(letter: char) -> Self {
        match letter {
            'X' => LOSE,
            'Y' => DRAW,
            'Z' => WIN,
            _ => panic!("No idea: {}", letter),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Selection {
    ROCK,
    PAPER,
    SCISSORS,
}

impl Selection {
    fn to_selection(letter: char) -> Self {
        match letter {
            'A' | 'X' => ROCK,
            'B' | 'Y' => PAPER,
            'C' | 'Z' => SCISSORS,
            _ => panic!("No idea: {}", letter),
        }
    }

    fn score(&self, other: &Selection) -> i32 {
        let mut score = 0;

        match self {
            ROCK => score += 1,
            PAPER => score += 2,
            SCISSORS => score += 3,
        }

        // Winning combos
        if (*self == ROCK && *other == SCISSORS) ||
            (*self == PAPER && *other == ROCK) ||
            (*self == SCISSORS && *other == PAPER) {
            score += 6
        }

        if self == other { score += 3 }

        score
    }

    fn pick(outcome: Outcome, opponent: &Selection) -> Self {
        match outcome {
            LOSE => {
                match opponent {
                    ROCK => { SCISSORS }
                    PAPER => { ROCK }
                    SCISSORS => { PAPER }
                }
            }
            DRAW => {
                opponent.clone()
            }
            WIN => {
                match opponent {
                    ROCK => { PAPER }
                    PAPER => { SCISSORS }
                    SCISSORS => { ROCK }
                }
            }
        }
    }
}