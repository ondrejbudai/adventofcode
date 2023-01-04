use std::collections::HashSet;
use std::io::{BufRead, stdin};
use std::iter::zip;
use std::str::FromStr;

enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::Right),
            "U" => Ok(Direction::Up),
            "L" => Ok(Direction::Left),
            "D" => Ok(Direction::Down),
            _ => Err(())
        }
    }
}

fn main() {
    let mut rope: [(i32, i32); 10] = [(0i32, 0i32); 10];
    let mut visited = HashSet::new();
    visited.insert(*rope.last().unwrap());

    stdin().lock().lines().for_each(|line| {
        let l = line.unwrap();
        let mut it = l.split(" ");
        let dir = it.next().unwrap().parse::<Direction>().unwrap();
        let steps = it.next().unwrap().parse::<u32>().unwrap();


        for _ in 0..steps {
            let head = &mut rope[0];
            match dir {
                Direction::Right => { (*head).0 += 1 }
                Direction::Up => { (*head).1 -= 1 }
                Direction::Left => { (*head).0 -= 1 }
                Direction::Down => { (*head).1 += 1 }
            }

            for(head_index, tail_index) in zip(0..rope.len()-1, 1..rope.len()) {
                let head = rope[head_index];
                let tail = &mut rope[tail_index];
                let delta0 = head.0 - tail.0;
                let delta1 = head.1 - tail.1;
                if delta0.abs() > 1 || delta1.abs() > 1 {
                    tail.0 += delta0.signum();
                    tail.1 += delta1.signum();
                }
            }
            visited.insert(*rope.last().unwrap());
        }
    });

    println!("{}", visited.len());
}
