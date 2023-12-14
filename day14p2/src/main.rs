use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, write};
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
enum Tile {
    Cube,
    Sphere,
    Empty,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Tile::Cube => '#',
            Tile::Sphere => 'O',
            Tile::Empty => '.'
        })
    }
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

enum Direction {
    North,
    West,
    South,
    East,
}

fn mv(map: &mut Vec<Vec<Tile>>, dir: Direction) {
    match dir {
        Direction::North => {
            for y in 1..map.len() {
                for x in 0..map[0].len() {
                    if map[y][x] != Tile::Sphere {
                        continue;
                    }
                    let mut new_y = y;
                    for yy in (0..y).rev() {
                        if map[yy][x] != Tile::Empty {
                            break;
                        }
                        new_y = yy;
                    }
                    map[y][x] = Tile::Empty;
                    map[new_y][x] = Tile::Sphere;
                }
            }
        }
        Direction::West => {
            for x in 1..map[0].len() {
                for y in 0..map.len() {
                    if map[y][x] != Tile::Sphere {
                        continue;
                    }
                    let mut new_x = x;
                    for xx in (0..x).rev() {
                        if map[y][xx] != Tile::Empty {
                            break;
                        }
                        new_x = xx;
                    }
                    map[y][x] = Tile::Empty;
                    map[y][new_x] = Tile::Sphere;
                }
            }
        }
        Direction::South => {
            for y in (0..map.len()-1).rev() {
                for x in 0..map[0].len() {
                    if map[y][x] != Tile::Sphere {
                        continue;
                    }
                    let mut new_y = y;
                    for yy in y+1..map.len() {
                        if map[yy][x] != Tile::Empty {
                            break;
                        }
                        new_y = yy;
                    }
                    map[y][x] = Tile::Empty;
                    map[new_y][x] = Tile::Sphere;
                }
            }
        }
        Direction::East => {
            for x in (0..map[0].len()-1).rev() {
                for y in 0..map.len() {
                    if map[y][x] != Tile::Sphere {
                        continue;
                    }
                    let mut new_x = x;
                    for xx in x+1..map[0].len() {
                        if map[y][xx] != Tile::Empty {
                            break;
                        }
                        new_x = xx;
                    }
                    map[y][x] = Tile::Empty;
                    map[y][new_x] = Tile::Sphere;
                }
            }

        }
    }
}

fn print(map: &Vec<Vec<Tile>>) {
    map.iter().for_each(|row| {
        row.iter().for_each(|t| {
            print!("{t}")
        });
        println!();
    });
    println!();
}

fn main() {
    let f = File::open("input").unwrap();
    let lines = BufReader::new(f).lines();

    let mut map: Vec<Vec<Tile>> = lines.map(|line| {
        line.unwrap().chars().map(|c| Tile::from(c)).collect()
    }).collect();
    print(&map);

    let mut states = HashMap::new();
    states.insert(map.clone(), (0, weight(&map)));
    let mut maps = vec![map.clone()];

    let mut start_loop = 0usize;
    let mut next_iter = 0usize;

    for i in 0..1000000000 {
        mv(&mut map, Direction::North);
        // print(&map);
        mv(&mut map, Direction::West);
        // print(&map);
        mv(&mut map, Direction::South);
        // print(&map);
        mv(&mut map, Direction::East);
        // print(&map);

        if states.contains_key(&map) {
            start_loop = states.get(&map).unwrap().0;
            next_iter = i;
            break;
        }

        states.insert(map.clone(), (i, weight(&map)));
        maps.push(map.clone());

    }

    let loop_len = next_iter - start_loop;
    let index = start_loop + ((1000000000 - start_loop) % loop_len);

    println!("{}", weight(&maps[index]));
}

fn weight(map: &Vec<Vec<Tile>>) -> usize {
    map.iter().rev().enumerate().map(|(y, row)| {
        row.iter().filter(|t| **t == Tile::Sphere).count() * (y + 1)
    }).sum()
}
