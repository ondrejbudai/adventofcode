use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader,BufRead};

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(f);
    let count: u32 = f.lines().map(|line| {
        let line = line.unwrap();
        let (_, line) = line.split_once(": ").unwrap();
        let (winning, mine) = line.split_once(" | ").unwrap();
        let winning = winning.split_whitespace().map(|x| x.parse().unwrap()).collect::<HashSet<u32>>();
        let mine = mine.split_whitespace().map(|x| x.parse().unwrap()).collect::<HashSet<u32>>();

        let matches = winning.intersection(&mine).count();
        if matches == 0 {
            return 0
        }

        2_u32.pow(matches as u32 - 1)
    }).sum();

    println!("{count}");
}
