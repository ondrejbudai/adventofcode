use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

enum Direction {
    Right,
    Left,
}

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(f);
    let mut lines = f.lines();

    let directions: Vec<Direction> = lines.next().unwrap().unwrap().chars().map(|x| {
        match x {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("unknown direction!")
        }
    }).collect();

    lines.next();
    let map: HashMap<String, (String, String)> =  lines.map(|l| {
        let l = l.unwrap();
        let (origin, options) = l.split_once(" = ").unwrap();
        let (left, right) = options[1..options.len()-1].split_once(", ").unwrap();
        (origin.to_string(), (left.to_string(), right.to_string()))
    }).collect();

    let mut steps_taken = 0;
    let mut next_step = 0;
    let mut cur = "AAA";

    loop {
        if cur == "ZZZ" {
            break;
        }
        let (left, right) = map.get(cur).unwrap();
        cur = match directions[next_step] {
            Direction::Left => left,
            Direction::Right => right,
        };
        steps_taken += 1;
        next_step = (next_step + 1) % directions.len();
    }

    println!("{steps_taken}");
}
