use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use num::integer::lcm;

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

    let steps_needed: u64 = map.iter().map(|(node, _)| node).filter(|node| node.chars().last().unwrap() == 'A').map(|start| {
        let mut steps_taken = 0;
        let mut next_step = 0;
        let mut cur = start.as_str();

        loop {
            if cur.chars().last().unwrap() == 'Z' {
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

        steps_taken
    }).fold(1, |acc, b| lcm(acc, b));

    println!("{steps_needed}");
}
