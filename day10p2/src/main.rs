use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::ops::AddAssign;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
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

#[derive(Eq, PartialEq, Copy, Clone)]
enum State {
    Out,
    GettingInCameNorth,
    GettingInCameSouth,
    GettingOutCameNorth,
    GettingOutCameSouth,
    In,
}

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(f);

    let mut map: Vec<Vec<Tile>> = f.lines().map(|line| {
        let line = line.unwrap();
        line.chars().map(|c| c.into()).collect()
    }).collect();

    let (start, start_tile, lop) = get_loop(&map);

    map[start.y as usize][start.x as usize] = start_tile;
    let map = map;

    let mut count = 0;

    for (y, line) in map.iter().enumerate() {
        let mut state = State::Out;
        for (x, t) in line.iter().enumerate() {
            if !lop.contains(&Coords { x: x as i32, y: y as i32 }) {
                if state == State::In {
                    count += 1;
                }
                continue;
            }
            state = match (state, t) {
                (State::Out, Tile::NorthEast) => State::GettingInCameNorth,
                (State::Out, Tile::NorthSouth) => State::In,
                (State::Out, Tile::SouthEast) => State::GettingInCameSouth,

                (State::GettingInCameNorth, Tile::NorthWest) => State::Out,
                (State::GettingInCameNorth, Tile::SouthWest) => State::In,
                (State::GettingInCameNorth, Tile::EastWest) => State::GettingInCameNorth,

                (State::GettingInCameSouth, Tile::NorthWest) => State::In,
                (State::GettingInCameSouth, Tile::SouthWest) => State::Out,
                (State::GettingInCameSouth, Tile::EastWest) => State::GettingInCameSouth,

                (State::In, Tile::NorthEast) => State::GettingOutCameNorth,
                (State::In, Tile::NorthSouth) => State::Out,
                (State::In, Tile::SouthEast) => State::GettingOutCameSouth,

                (State::GettingOutCameNorth, Tile::NorthWest) => State::In,
                (State::GettingOutCameNorth, Tile::SouthWest) => State::Out,
                (State::GettingOutCameNorth, Tile::EastWest) => State::GettingOutCameNorth,

                (State::GettingOutCameSouth, Tile::NorthWest) => State::Out,
                (State::GettingOutCameSouth, Tile::SouthWest) => State::In,
                (State::GettingOutCameSouth, Tile::EastWest) => State::GettingOutCameSouth,

                (_, _) => panic!("nonsense transition")
            }
        }
    }

    println!("{count}");
}

fn get_loop(map: &Vec<Vec<Tile>>) -> (Coords, Tile, HashSet<Coords>) {
    let start = find_start(&map);
    let (mut dir, start_tile) = find_dir(&map, &start);
    let mut cur = start;

    let mut lop: HashSet<Coords> = HashSet::new();
    lop.insert(start);

    loop {
        let delta = step(&dir);
        cur += delta;
        lop.insert(cur);

        if cur == start {
            break;
        }

        dir = next_dir(dir, map[cur.y as usize][cur.x as usize]).unwrap();
    }
    (start, start_tile, lop)
}

fn find_dir(map: &Vec<Vec<Tile>>, c: &Coords) -> (Dir, Tile) {
    let mut dirs = vec![];
    if c.x > 0 && [Tile::EastWest, Tile::NorthEast, Tile::SouthEast].contains(&map[c.y as usize][(c.x as usize) - 1]) {
        dirs.push(Dir::West);
    }
    if (c.x as usize) < map[c.y as usize].len() - 1 && [Tile::EastWest, Tile::NorthWest, Tile::SouthWest].contains(&map[c.y as usize][(c.x as usize) + 1]) {
        dirs.push(Dir::East);
    }
    if c.y > 0 && [Tile::SouthWest, Tile::SouthEast, Tile::NorthSouth].contains(&map[(c.y as usize) - 1][c.x as usize]) {
        dirs.push(Dir::North);
    }
    if (c.y as usize) < map.len() - 1 && [Tile::NorthSouth, Tile::NorthEast, Tile::NorthWest].contains(&map[(c.y as usize) + 1][c.x as usize]) {
        dirs.push(Dir::South);
    }

    if dirs.len() != 2 {
        panic!("cannot find initial dir");
    }

    match (dirs[0], dirs[1]) {
        (Dir::West, Dir::East) => (dirs[0], Tile::EastWest),
        (Dir::West, Dir::North) => (dirs[0], Tile::NorthWest),
        (Dir::West, Dir::South) => (dirs[0], Tile::SouthWest),
        (Dir::East, Dir::North) => (dirs[0], Tile::NorthEast),
        (Dir::East, Dir::South) => (dirs[0], Tile::SouthEast),
        (Dir::North, Dir::South) => (dirs[0], Tile::NorthSouth),
        (_, _) => panic!("cannot infer initial tile")
    }
}

fn find_start(map: &Vec<Vec<Tile>>) -> Coords {
    for (y, l) in map.iter().enumerate() {
        for (x, t) in l.iter().enumerate() {
            if *t == Tile::Start {
                return Coords { x: x as i32, y: y as i32 };
            }
        }
    }
    panic!("cannot find start")
}
