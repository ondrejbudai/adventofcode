use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader,BufRead};

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(f);
    let mut copies: HashMap<usize, u32> = HashMap::new();
    let count: u32 = f.lines().enumerate().map(|(id, line)| {
        let line = line.unwrap();
        let (_, line) = line.split_once(": ").unwrap();
        let (winning, mine) = line.split_once(" | ").unwrap();
        let winning = winning.split_whitespace().map(|x| x.parse().unwrap()).collect::<HashSet<u32>>();
        let mine = mine.split_whitespace().map(|x| x.parse().unwrap()).collect::<HashSet<u32>>();

        let cur_copies = *copies.entry(id).or_insert(1);

        let matches = winning.intersection(&mine).count();

        for i in 1..=matches {
            let c = copies.entry(id + i).or_insert(1);
            *c += cur_copies;
        }

        cur_copies


    }).sum();
    println!("{count}");
}
