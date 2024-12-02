use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};
use priority_queue::PriorityQueue;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct State {
    x: i32,
    y: i32,
    dir: Direction,
    steps_taken: u8,
}

fn turns(d: Direction) -> [Direction; 2] {
    match d {
        Direction::Left => [Direction::Up, Direction::Down],
        Direction::Right => [Direction::Up, Direction::Down],
        Direction::Up => [Direction::Left, Direction::Right],
        Direction::Down => [Direction::Left, Direction::Right]
    }
}

fn next_states(s: &State, map: &HashMap<(i32, i32), u32>) -> Vec<State> {
    let mut dirs = vec![];

    if s.steps_taken < 10 {
        dirs.push(s.dir);
    }

    if s.steps_taken > 3 || s.steps_taken == 0 {
        dirs.extend_from_slice(&turns(s.dir));
    }

    dirs.iter().map(|d| mv(s, d)).filter(|s| map.contains_key(&(s.x, s.y))).collect()
}

fn mv(s: &State, new_dir: &Direction) -> State {
    let mut new_state = s.clone();
    match new_dir {
        Direction::Left => {
            new_state.x -= 1;
        }
        Direction::Right => {
            new_state.x += 1;
        }
        Direction::Up => {
            new_state.y -= 1;
        }
        Direction::Down => {
            new_state.y += 1;
        }
    }

    new_state.steps_taken += 1;
    new_state.dir = *new_dir;
    if s.dir != *new_dir {
        new_state.steps_taken = 1;
    }


    new_state
}

fn main() {
    let f = File::open("input").unwrap();
    let lines = BufReader::new(f).lines();

    let mut max_x = 0;
    let mut max_y = 0;

    let map: HashMap<(i32, i32), u32> = lines.enumerate().flat_map(|(y, l)| {
        let l = l.unwrap();
        max_y = max_y.max(y as i32);
        l.chars().enumerate().map(|(x, c)| {
            max_x = max_x.max(x as i32);
            ((x as i32, y as i32), c.to_digit(10).unwrap())
        }).collect::<Vec<_>>()
    }).collect();

    let mut to_visit = PriorityQueue::new();
    to_visit.push(State {
        x: 0,
        y: 0,
        dir: Direction::Right,
        steps_taken: 0,
    }, Reverse(0));

    let mut visited = HashSet::new();
    visited.insert(to_visit.peek().unwrap().0.clone());

    loop {
        let (s, heat) = to_visit.pop().unwrap();
        let heat = heat.0;

        if s.x == max_x && s.y == max_y && s.steps_taken > 3{
            println!("{heat}");
            break;
        }

        for new_s in next_states(&s, &map) {
            if visited.contains(&new_s) {
                continue;
            }
            let new_heat = heat + map.get(&(new_s.x, new_s.y)).unwrap();
            to_visit.push(new_s, Reverse(new_heat));
            visited.insert(new_s);
        }
    }
}
