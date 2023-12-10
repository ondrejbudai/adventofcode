use std::fs::File;
use std::io::{BufReader, BufRead};
use std::ops::AddAssign;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Coords {
    x: i32,
    y: i32,
}

impl AddAssign for Coords {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Dir {
    North,
    East,
    South,
    West,
}

fn next_dir(dir: Dir, tile: Tile) -> Option<Dir> {
    match (dir, tile) {
        (Dir::North, Tile::NorthSouth) => Some(Dir::North),
        (Dir::North, Tile::SouthWest) => Some(Dir::West),
        (Dir::North, Tile::SouthEast) => Some(Dir::East),
        (Dir::East, Tile::EastWest) => Some(Dir::East),
        (Dir::East, Tile::NorthWest) => Some(Dir::North),
        (Dir::East, Tile::SouthWest) => Some(Dir::South),
        (Dir::South, Tile::NorthSouth) => Some(Dir::South),
        (Dir::South, Tile::NorthEast) => Some(Dir::East),
        (Dir::South, Tile::NorthWest) => Some(Dir::West),
        (Dir::West, Tile::EastWest) => Some(Dir::West),
        (Dir::West, Tile::SouthEast) => Some(Dir::South),
        (Dir::West, Tile::NorthEast) => Some(Dir::North),
        (_, _) => None
    }
}

fn step(dir: &Dir) -> Coords {
    match dir {
        Dir::North => Coords { x: 0, y: -1 },
        Dir::East => Coords { x: 1, y: 0 },
        Dir::South => Coords { x: 0, y: 1 },
        Dir::West => Coords { x: -1, y: 0 },
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Tile::NorthSouth,
            '-' => Tile::EastWest,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!("unknown tile!"),
        }
    }
}

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(f);

    let map: Vec<Vec<Tile>> = f.lines().map(|line| {
        let line = line.unwrap();
        line.chars().map(|c| c.into()).collect()
    }).collect();
    let start = find_start(&map);
    let mut dir = find_dir(&map, &start);
    let mut cur = start;
    let mut steps = 0;

    loop {
        let delta = step(&dir);
        cur += delta;

        steps += 1;
        if cur == start {
            break;
        }

        dir = next_dir(dir, map[cur.y as usize][cur.x as usize]).unwrap();

    }

    println!("{}", steps / 2);
}

fn find_dir(map: &Vec<Vec<Tile>>, c: &Coords) -> Dir {
    if c.x > 0 && [Tile::EastWest, Tile::NorthEast, Tile::SouthEast].contains(&map[c.y as usize][(c.x as usize) - 1]) {
        return Dir::West;
    }
    if (c.x as usize) < map[c.y as usize].len() - 1 && [Tile::EastWest, Tile::NorthWest, Tile::SouthWest].contains(&map[c.y as usize][(c.x as usize) + 1]) {
        return Dir::East;
    }
    if c.y > 0 && [Tile::SouthWest, Tile::SouthEast, Tile::NorthSouth].contains(&map[(c.y as usize) - 1][c.x as usize]) {
        return Dir::North;
    }
    if (c.y as usize) < map.len() - 1 && [Tile::NorthSouth, Tile::NorthEast, Tile::NorthWest].contains(&map[(c.y as usize) + 1][c.x as usize]) {
        return Dir::South;
    }

    panic!("cannot find initial dir");
}

fn find_start(map: &Vec<Vec<Tile>>) -> Coords {
    for (y, l) in map.iter().enumerate(){
        for (x, t) in l.iter().enumerate(){
            if *t == Tile::Start {
                return Coords { x: x as i32, y: y as i32 }
            }
        }
    }
    panic!("cannot find start")
}
