use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};
use glob::Pattern;
use regex::Regex;

struct Solver {
    record: String,
    counts: Vec<u32>,
    cache: HashMap<(usize, usize), u64>,
}

impl Solver {
    fn new(record: String, counts: Vec<u32>) -> Self {
        return Solver {
            record,
            counts,
            cache: HashMap::new(),
        };
    }
    fn solve(&mut self, next_char: usize, next_count: usize) -> u64 {
        if let Some(cached) = self.cache.get(&(next_char, next_count)) {
            return *cached;
        }
        let mut sum = 0;
        if next_count == self.counts.len() {
            if next_char == self.record.len() || !self.record[next_char..].contains("#") {
                sum = 1;
            }
        } else if self.counts[next_count] as usize + next_char <= self.record.len() {
            if !self.record[next_char..next_char + self.counts[next_count] as usize].contains(".") {
                if next_char + self.counts[next_count] as usize == self.record.len() {
                    if next_count + 1 == self.counts.len() {
                        sum = 1;
                    }
                } else if self.record.chars().nth(next_char + self.counts[next_count] as usize).unwrap() != '#' {
                    sum += self.solve(next_char + self.counts[next_count] as usize + 1, next_count + 1);
                }
            }

            if self.record.chars().nth(next_char).unwrap() != '#' {
                sum += self.solve(next_char + 1, next_count);
            }
        }

        self.cache.insert((next_char, next_count), sum);
        sum
    }
}

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(f);

    let factor = 5;

    let sum: u64 = f.lines().enumerate().map(|(i, line)| {
        let line = line.unwrap();

        let (record, damaged_counts) = line.split_once(" ").unwrap();
        let damaged_counts: Vec<u32> = damaged_counts.split(",").map(|x| x.parse().unwrap()).collect();

        let record = (0..factor).map(|_| record).collect::<Vec<_>>().join("?");

        let mut damaged_counts = damaged_counts.repeat(factor);

        let sum = Solver::new(record, damaged_counts).solve(0, 0);


        sum
    }).sum();

    println!("{sum}");
}
