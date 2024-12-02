use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(f);

    let mut add_row = vec![];
    let mut map: Vec<Vec<char>> = f.lines().enumerate().map(|(y, line)| {
        let line = line.unwrap();
        if !line.contains("#") {
            add_row.push(y);
        }
        line.chars().collect()
    }).collect();

    let mut add_col = vec![];
    'outer:
    for x in 0..map[0].len(){
        for y in 0..map.len() {
            if map[y][x] == '#'{
                continue 'outer;
            }
        }

        add_col.push(x);
    }

    let mut galaxies: HashSet<(usize, usize)> = HashSet::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '#' {
                galaxies.insert((x, y));
            }
        }
    }

    let galaxies: Vec<(usize, usize)> = galaxies.iter().map(|(x,y)| {
        let xd = add_col.iter().filter(|xx| {
            x > xx
        }).count();
        let yd = add_row.iter().filter(|yy| {
            y > yy
        }).count();

        (x+xd*(999999), y+yd*999999)
    }).collect();

    let sum: usize = galaxies.iter().tuple_combinations().map(|(a,b)| {
        ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize
    }).sum();

    println!("{sum}");
}
