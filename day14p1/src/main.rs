use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Tile {
    Cube,
    Sphere,
    Empty
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Tile::Cube,
            'O' => Tile::Sphere,
            '.' => Tile::Empty,
            _ => panic!("unknown tile!")
        }
    }
}

fn mv(map: &Vec<Vec<Tile>>) -> (Vec<Vec<Tile>>, usize) {
    let mut new_map = vec![map[0].clone()];

    let mut moved = 0usize;

    for y in 1..map.len() {
        let new_row = vec![Tile::Empty];
        new_map.push(new_row.repeat(map[y].len()));
        for x in 0..map[y].len() {
            if map[y][x] != Tile::Sphere {
                new_map[y][x] = map[y][x];
                continue;
            }
            if new_map[y-1][x] == Tile::Empty {
                new_map[y-1][x] = Tile::Sphere;
                moved += 1;
            } else {
                new_map[y][x] = Tile::Sphere;
            }
        }
    }

    (new_map, moved)
}

fn main() {
    let f = File::open("input").unwrap();
    let lines = BufReader::new(f).lines();

    let mut map: Vec<Vec<Tile>> = lines.map(|line| {
        line.unwrap().chars().map(|c| Tile::from(c)).collect()
    }).collect();

    loop {
        let count;
        (map, count) = mv(&map);
        if count == 0 {
            break;
        }
    }

    let sum: usize = map.iter().rev().enumerate().map(|(y, row)| {
        row.iter().filter(|t| **t == Tile::Sphere).count() * (y + 1)
    }).sum();

    println!("{sum}");


}
