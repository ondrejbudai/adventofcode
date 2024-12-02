use std::cmp::min;
use std::collections::{HashSet, VecDeque};
use std::io::{BufRead, stdin};

fn main() {
    let mut map: Vec<Vec<u16>> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    stdin().lock().lines().map(|l| l.unwrap()).enumerate().for_each(|(row, line)| {
        let mut row_vec: Vec<u16> = Vec::new();
        line.as_bytes()
            .iter()
            .enumerate()
            .for_each(|(col, h)| {
                if *h == 'S' as u16 {
                    start = (row, col);
                    row_vec.push('a' as u16 - 'a' as u16);
                } else if *h == 'E' as u16 {
                    end = (row, col);
                    row_vec.push('z' as u16 - 'a' as u16);
                } else {
                    row_vec.push(h - 'a' as u16);
                }
            });

        map.push(row_vec);
    });

    println!("{}", dijkstra(&map, start, end).unwrap());

    let mut min_path = 10000;
    map.iter().enumerate().for_each(|(row, cols)| {
        cols.iter().enumerate().for_each(|(col, height)| {
            if *height == 0 {
                let path_length = dijkstra(&map, (row, col), end);
                if let Some(length) = path_length {
                    min_path = min(min_path, length);
                }
            }
        });
    });

    println!("{}", min_path);

}

fn dijkstra(map: &Vec<Vec<u16>>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut to_visit: VecDeque<(usize, usize, usize)> = VecDeque::new();
    to_visit.push_back((start.0, start.1, 0));
    visited.insert((start.0, start.1));

    loop {
        if to_visit.len() == 0 {
            return None;
        }
        let cur = to_visit.pop_front().unwrap();
        if cur.0 == end.0 && cur.1 == end.1 {
            return Some(cur.2);
        }
        let cur_height = map[cur.0][cur.1];
        if cur.0 > 0 && !visited.contains(&(cur.0 - 1, cur.1)) {
            if let Some(height) = map.get(cur.0 - 1).and_then(|v| v.get(cur.1)) {
                if *height <= cur_height + 1 {
                    to_visit.push_back((cur.0 - 1, cur.1, cur.2 + 1));
                    visited.insert((cur.0 - 1, cur.1));
                }
            }
        }
        if !visited.contains(&(cur.0 + 1, cur.1)) {
            if let Some(height) = map.get(cur.0 + 1).and_then(|v| v.get(cur.1)) {
                if *height <= cur_height + 1 {
                    to_visit.push_back((cur.0 + 1, cur.1, cur.2 + 1));
                    visited.insert((cur.0 + 1, cur.1));
                }
            }
        }
        if cur.1 > 0 && !visited.contains(&(cur.0, cur.1 - 1)) {
            if let Some(height) = map.get(cur.0).and_then(|v| v.get(cur.1 - 1)) {
                if *height <= cur_height + 1 {
                    to_visit.push_back((cur.0, cur.1 - 1, cur.2 + 1));
                    visited.insert((cur.0, cur.1 - 1));
                }
            }
        }
        if !visited.contains(&(cur.0, cur.1 + 1)) {
            if let Some(height) = map.get(cur.0).and_then(|v| v.get(cur.1 + 1)) {
                if *height <= cur_height + 1 {
                    to_visit.push_back((cur.0, cur.1 + 1, cur.2 + 1));
                    visited.insert((cur.0, cur.1 + 1));
                }
            }
        }
    }
}
