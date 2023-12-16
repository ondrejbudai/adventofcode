use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    Empty,
    // /
    ForwardMirror,
    // \
    BackMirror,
    Vertical,
    Horizontal,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '/' => Self::ForwardMirror,
            '\\' => Self::BackMirror,
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            _ => panic!("unknown character")
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Beam {
    x: i32,
    y: i32,
    dir: Direction,
}

fn dirs(old_dir: Direction, tile: &Tile) -> Vec<Direction> {
    match tile {
        Tile::Empty => vec![old_dir],
        Tile::ForwardMirror => {
            match old_dir {
                Direction::Left => vec![Direction::Down],
                Direction::Right => vec![Direction::Up],
                Direction::Up => vec![Direction::Right],
                Direction::Down => vec![Direction::Left],
            }
        }
        Tile::BackMirror => {
            match old_dir {
                Direction::Left => vec![Direction::Up],
                Direction::Right => vec![Direction::Down],
                Direction::Up => vec![Direction::Left],
                Direction::Down => vec![Direction::Right],
            }
        }
        Tile::Vertical => {
            match old_dir {
                Direction::Left => vec![Direction::Up, Direction::Down],
                Direction::Right => vec![Direction::Up, Direction::Down],
                Direction::Up => vec![Direction::Up],
                Direction::Down => vec![Direction::Down],
            }
        }
        Tile::Horizontal => {
            match old_dir {
                Direction::Left => vec![Direction::Left],
                Direction::Right => vec![Direction::Right],
                Direction::Up => vec![Direction::Left, Direction::Right],
                Direction::Down => vec![Direction::Left, Direction::Right],
            }
        }
    }
}

fn mv(x: i32, y: i32, dir: Direction) -> (i32, i32) {
    match dir {
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
    }
}

fn main() {
    let f = File::open("input").unwrap();
    let lines = BufReader::new(f).lines();

    let map: HashMap<(i32, i32), Tile> = lines.enumerate().flat_map(|(y, l)| {
        let l = l.unwrap();
        l.chars().enumerate().map(|(x, c)| ((x as i32, y as i32), Tile::from(c))).collect::<Vec<_>>()
    }).collect();

    let mut beams = vec![Beam {
        x: 0,
        y: 0,
        dir: Direction::Right,
    }];
    let mut beams_done = HashSet::new();
    beams_done.insert(beams[0]);
    let mut energized = HashSet::new();

    while !beams.is_empty() {
        let b = beams.pop().unwrap();
        energized.insert((b.x, b.y));

        let new_dirs = dirs(b.dir, map.get(&(b.x, b.y)).unwrap());

        for dir in new_dirs {
            let (x, y) = mv(b.x, b.y, dir);
            if !map.contains_key(&(x, y)) {
                continue;
            }
            let b = Beam {x, y, dir};
            if beams_done.contains(&b) {
                continue;
            }
            beams_done.insert(b);
            beams.push(b);
        }
    }

    println!("{}", energized.len());
}
