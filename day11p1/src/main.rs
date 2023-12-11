use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(f);

    let mut map: Vec<Vec<char>> = f.lines().map(|line| {
        let line = line.unwrap();
        if !line.contains('#') {
            return vec![line.chars().collect(), line.chars().collect()];
        }
        vec![line.chars().collect()]
    }).flatten().collect();

    let mut add = vec![];
    'outer:
    for x in 0..map[0].len(){
        for y in 0..map.len() {
            if map[y][x] == '#'{
                continue 'outer;
            }
        }

        add.push(x);
    }

    for (delta, x) in add.iter().enumerate() {
        for y in 0..map.len() {
            map[y].insert(*x + delta, '.');
        }
    }

    let mut galaxies: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '#' {
                galaxies.insert((x, y));
            }
        }
    }

    let sum: usize = galaxies.iter().tuple_combinations().map(|(a,b)| {
        ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize
    }).sum();

    println!("{sum}");
}
