use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{BufRead, stdin};
use std::ops::{Add, AddAssign};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Coords(i32, i32);

impl AddAssign for Coords {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Add for Coords {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coords(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn next_wind(wind: &Vec<(Coords, char)>, rows: i32, cols: i32) -> Vec<(Coords, char)> {
    wind.iter().map(|(coords, dir)| {
        let mut new_coords;
        if *dir == '>' {
            new_coords = Coords(coords.0, coords.1 + 1);
        } else if *dir == 'v' {
            new_coords = Coords(coords.0 + 1, coords.1);
        } else if *dir == '<' {
            new_coords = Coords(coords.0, coords.1 - 1);
        } else if *dir == '^' {
            new_coords = Coords(coords.0 - 1, coords.1);
        } else {
            unreachable!();
        }
        if new_coords.0 == 0 {
            new_coords.0 = rows - 2;
        }
        if new_coords.0 == rows - 1 {
            new_coords.0 = 1;
        }
        if new_coords.1 == 0 {
            new_coords.1 = cols - 2;
        }
        if new_coords.1 == cols - 1 {
            new_coords.1 = 1;
        }

        (new_coords, *dir)
    }).collect::<Vec<_>>()
}

fn print(wind: &Vec<(Coords, char)>, rows: i32, cols: i32) {
    for row in 0..rows {
        for col in 0..cols {
            if let Some((_, dir)) = wind.iter().find(|(coords, _)| *coords == Coords(row, col)) {
                print!("{}", dir);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let start = Coords(0, 1);
    let mut wind = Vec::new();

    let mut rows = 0;
    let mut cols = 0;

    stdin().lock().lines().enumerate().for_each(|(row, data)| {
        rows += 1;
        let data = data.unwrap();
        cols = data.len() as i32;
        data.chars().enumerate().for_each(|(col, c)| {
            if c != '#' && c != '.' {
                wind.push((Coords(row as i32, col as i32), c));
            }
        });
    });
    print(&wind, rows, cols);
    println!();

    let end = Coords(rows - 1, cols - 2);
    let mut to_visit = VecDeque::new();
    let mut visited = HashSet::<(Coords, usize, u8)>::new();

    to_visit.push_back((start, 0, 3u8));
    visited.insert((start, 0, 3u8));

    let mut winds = Vec::new();
    winds.push(wind);

    let mut best = usize::MAX;

    while to_visit.len() > 0 {
        let (cur_coords, cur_time, mut remaining_goals) = to_visit.pop_front().unwrap();
        if cur_coords == end && remaining_goals == 1 {
            if cur_time < best {
                best = cur_time;
                println!("{cur_time}");
            }
            continue;
        } else if cur_coords == end && remaining_goals == 3 {
            remaining_goals = 2;
        } else if cur_coords == start && remaining_goals == 2 {
            remaining_goals = 1;
        }

        if cur_time >= best {
            continue;
        }

        if winds.len() == cur_time + 1 {
            winds.push(next_wind(winds.last().unwrap(), rows, cols));
            // print(winds.last().unwrap(), rows, cols);
            // println!();
        }

        let wind = &winds[cur_time + 1];


        let nexts = [
            cur_coords + Coords(0, 1),
            cur_coords + Coords(1, 0),
            cur_coords + Coords(0, -1),
            cur_coords + Coords(-1, 0),
            cur_coords + Coords(-1, 0),
        ];

        for next in nexts.iter() {
            if *next != end && *next != start && (next.0 < 1 || next.1 < 1 || next.0 > rows - 2 || next.1 > cols - 2 || wind.iter().find(|(coords, _)| *coords == *next).is_some()) {
                continue;
            }

            if !visited.contains(&(*next, cur_time + 1, remaining_goals)) {
                to_visit.push_back((*next, cur_time + 1, remaining_goals));
                visited.insert((*next, cur_time + 1, remaining_goals));
            }
        }

        if wind.iter().find(|(coords, _)| *coords == cur_coords).is_none() {
            if !visited.contains(&(cur_coords, cur_time + 1, remaining_goals)) {
                to_visit.push_back((cur_coords, cur_time + 1, remaining_goals));
                visited.insert((cur_coords, cur_time + 1, remaining_goals));
            }
        }
    }
    println!("{best}");
}
