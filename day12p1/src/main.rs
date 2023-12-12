use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};
use glob::Pattern;

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(f);

    let factor = 4;

    let sum: u32 = f.lines().enumerate().map(|(index, line)| {
        let line = line.unwrap();
        let (record, damaged_counts) = line.split_once(" ").unwrap();
        let damaged_counts: Vec<u32> = damaged_counts.split(",").map(|x| x.parse().unwrap()).collect();

        let record = (0..factor).map(|_| record).collect::<Vec<_>>().join("?");



        let damaged_counts = damaged_counts.repeat(factor);

        let mut spaces: Vec<u32> = Vec::new();
        spaces.push(0);
        for _ in 0..damaged_counts.len() - 1 {
            spaces.push(1);
        }

        spaces.push(0);

        let mut to_visit = vec![spaces.clone()];
        let mut visited = HashSet::new();
        visited.insert(spaces);

        let total: u32 = record.len() as u32;
        let damaged: u32 = damaged_counts.iter().sum();

        let mut sum = 0u32;
        let pat = Pattern::new(&record).unwrap();

        while to_visit.len() > 0 {
            let spaces = to_visit.pop().unwrap();
            let count_sum: u32 = spaces.iter().sum();
            if count_sum + damaged == total {
                let mut record_try = String::new();
                for i in 0..spaces.len() {
                    for _ in 0..spaces[i] {
                        record_try += ".";
                    }
                    if i == spaces.len() - 1 {
                        break;
                    }

                    for _ in 0..damaged_counts[i] {
                        record_try += "#";
                    }
                }

                if pat.matches(&record_try) {
                    sum += 1;
                }


                continue;
            }

            for i in 0..spaces.len() {
                let mut new = spaces.clone();
                new[i] += 1;
                if !visited.contains(&new) {
                    to_visit.push(new.clone());
                    visited.insert(new);
                }
            }
        }

        sum
    }).sum();

    println!("{sum}");
}
