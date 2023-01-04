use std::cmp::{max, min};
use std::collections::HashSet;
use std::io::{BufRead, stdin};
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::tuple;

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |out: &str| out.parse::<u32>())(input)
}

fn parse_tuple(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, (x, _ , y)) = tuple((parse_u32, tag(","), parse_u32))(input)?;
    Ok((input, (x, y)))
}

fn parse(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    separated_list1(tag(" -> "), parse_tuple)(input)
}

fn main() {
    let mut max_y= 0;
    let mut orig_map: HashSet<(u32, u32)> = HashSet::new();
    stdin().lock().lines().map(|l| {l.unwrap()}).for_each(|line| {
        let (_, points) = parse(line.as_str()).unwrap();
        let starts = 0..points.len()-1;
        let ends = 1..points.len();

        starts.zip(ends).for_each(|(start_index, end_index)| {
            let start = points.get(start_index).unwrap();
            let end = points.get(end_index).unwrap();

            let startx = min(start.0, end.0);
            let endx = max(start.0, end.0);

            let starty = min(start.1, end.1);
            let endy = max(start.1, end.1);

            for x in startx..=endx {
                for y in starty..=endy {
                    orig_map.insert((x, y));
                    max_y = max(max_y, y);
                }
            }
        });
    });

    let mut landed_count = 0;
    let start = (500u32,0u32);
    let mut active_sand = start;
    let mut map = orig_map.clone();

    loop {
        if active_sand.1 == max_y {
            break;
        }
        if let None = map.get(&(active_sand.0, active_sand.1+1)){
            active_sand.1 += 1;


        } else if let None = map.get(&(active_sand.0-1, active_sand.1+1)) {
            active_sand.0 -= 1;
            active_sand.1 += 1;
        } else if let None = map.get(&(active_sand.0+1, active_sand.1+1)) {
            active_sand.0 += 1;
            active_sand.1 += 1;
        } else {
            map.insert(active_sand);
            active_sand = start;
            landed_count += 1;
        }
    }

    println!("{}", landed_count);

    let mut landed_count = 0;
    let start = (500u32,0u32);
    let mut active_sand = start;
    let max_y = max_y + 2;
    let mut map = orig_map.clone();

    loop {
        if active_sand.1 + 1 == max_y {
            map.insert(active_sand);
            landed_count += 1;
            active_sand = start;

        } else if let None = map.get(&(active_sand.0, active_sand.1+1)){
            active_sand.1 += 1;
        } else if let None = map.get(&(active_sand.0-1, active_sand.1+1)) {
            active_sand.0 -= 1;
            active_sand.1 += 1;
        } else if let None = map.get(&(active_sand.0+1, active_sand.1+1)) {
            active_sand.0 += 1;
            active_sand.1 += 1;
        } else if active_sand == start {
            landed_count += 1;
            break;
        } else {
            map.insert(active_sand);
            landed_count += 1;
            active_sand = start;
        }
    }

    println!("{}", landed_count);
}
