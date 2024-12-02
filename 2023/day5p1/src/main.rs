use std::fs::File;
use std::io::{BufReader, BufRead};

struct Map {
    from: u32,
    to: u32,
    range: u32,
}

fn transition(maps: &Vec<Map>, from: u32) -> u32 {
    for map in maps {
        if from >= map.from && from - map.from < map.range {
            return map.to + (from - map.from);
        }
    }

    from
}

fn main() {
    let f = File::open("input").unwrap();
    let mut lines = BufReader::new(f).lines();

    let seeds: Vec<u32> = lines.next().unwrap().unwrap().split(" ").skip(1).map(|x | x.parse().unwrap()).collect();
    lines.next();

    let maps: Vec<_> = (0..7).map(|_| {
        lines.next();

        let mut maps = vec![];


        loop {
            let line = lines.next();

            if line.is_none() {
                break;
            }

            let line = line.unwrap().unwrap();

            if line == "" {
                break;
            }

            let (to, line) = line.split_once(" ").unwrap();
            let (from, range) = line.split_once(" ").unwrap();


            maps.push(Map{
                from: from.parse().unwrap(),
                to: to.parse().unwrap(),
                range: range.parse().unwrap(),
            });
        }

        maps
    }).collect();

    let min = seeds.iter().map(|seed| {
        let mut cur = *seed;
        for map in &maps {
            cur = transition(map, cur);
        }

        cur
    }).min().unwrap();

    println!("{min}");

}
