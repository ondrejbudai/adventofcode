use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct Map {
    from: u32,
    to: u32,
    range: u32,
}

fn back(maps: &Vec<Map>, to: u32) -> u32 {
    for map in maps {
        if to >= map.to && to - map.to < map.range {
            return map.from + (to - map.to);
        }
    }

    to
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

    let mut min = 0;

    'outer:
    loop {

        let mut cur = min;

        for map in maps.iter().rev() {
            cur = back(map, cur);
        }

        for seed_range in seeds.chunks(2) {
            let from = seed_range[0];
            let range = seed_range[1];

            if cur >= from && cur - from < range {
                break 'outer;
            }
        }


        min += 1;
    }

    println!("{min}");

}
