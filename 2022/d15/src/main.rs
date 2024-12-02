use std::cmp::{max, min};
use std::collections::HashSet;
use std::io::{BufRead, stdin};

use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::{eof, map_res, opt, recognize};
use nom::IResult;
use nom::sequence::pair;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}


impl Point {
    fn distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn to_weird(&self) -> WeirdPoint {
        WeirdPoint { x: self.x + self.y, y: -self.x + self.y }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct WeirdPoint {
    x: i32,
    y: i32,
}

impl WeirdPoint {
    fn to_point(&self) -> Point {
        Point { x: (self.x - self.y) / 2, y: (self.x + self.y) / 2 }
    }
}

#[derive(Debug)]
struct Box {
    top_left: WeirdPoint,
    bottom_right: WeirdPoint,
}

#[derive(Copy, Clone)]
struct PointAreaIterator {
    point: Point,
    distance: i32,
    current: Option<Point>,
}

impl PointAreaIterator {
    fn new(point: Point, distance: i32) -> PointAreaIterator {
        PointAreaIterator {
            point,
            distance,
            current: None,
        }
    }
}

impl Iterator for PointAreaIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current;
        if let None = self.current {
            current = Point { x: self.point.x - self.distance, y: self.point.y - self.distance };
        } else {
            current = self.current.unwrap();
        }
        let end = Point { x: self.point.x - self.distance, y: self.point.y + self.distance + 1 };
        while current != end {
            if current.x - self.point.x < self.distance {
                current.x += 1;
            } else {
                current.x = self.point.x - self.distance;
                current.y += 1;
            }

            if current.distance(&self.point) <= self.distance {
                self.current = Some(current);
                return self.current;
            }
        }

        None
    }
}

fn parse_i32(input: &str) -> IResult<&str, i32> {
    map_res(recognize(pair(opt(char('-')), digit1)), |o: &str| o.parse::<i32>())(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, _) = tag("x=")(input)?;
    let (input, x) = parse_i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y) = parse_i32(input)?;

    Ok((input, Point { x, y }))
}

fn parse(input: &str) -> IResult<&str, Sensor> {
    let (input, _) = tag("Sensor at ")(input)?;
    let (input, p1) = parse_point(input)?;
    let (input, _) = tag(": closest beacon is at ")(input)?;
    let (input, p2) = parse_point(input)?;
    eof(input)?;

    Ok((input, Sensor { position: p1, closest_beacon: p2 }))
}

struct Sensor {
    position: Point,
    closest_beacon: Point,
}

impl Sensor {
    fn to_box(&self) -> Box {
        let distance = self.position.distance(&self.closest_beacon);
        Box {
            top_left: Point { x: self.position.x, y: self.position.y - distance }.to_weird(),
            bottom_right: Point { x: self.position.x, y: self.position.y + distance }.to_weird(),
        }
    }
}

fn merge_interval(intervals: &Vec<(i32, i32)>, int: (i32, i32)) -> Vec<(i32, i32)> {
    let mut new: Vec<(i32, i32)> = Vec::new();
    let mut merged = int;
    intervals.iter().for_each(|cur| {
        if cur.1 > merged.0 && merged.1 > cur.0 {
            merged.0 = min(merged.0, cur.0);
            merged.1 = max(merged.1, cur.1);
        } else {
            new.push(*cur);
        }
    });

    new.push(merged);

    new.sort_by(|a, b| { a.cmp(b) });

    return new;
}

fn intervals_has_gaps(intervals: &Vec<(i32, i32)>) -> bool {
    if intervals.len() <= 1 {
        return false;
    }

    let mut last = intervals.get(0).unwrap().1;

    for i in 1..intervals.len() {
        let i = intervals.get(i).unwrap();

        if i.0 > last + 1 {
            return true;
        }

        last = i.1;
    }

    false
}

fn main() {
    let sensors = stdin().lock().lines().map(|l| l.unwrap()).map(|l| {
        let (_, sensor) = parse(l.as_str()).unwrap();
        sensor
    }).collect::<Vec<Sensor>>();

    // let beacons = sensors.iter().map(|s| s.closest_beacon).collect::<HashSet<Point>>();
    //
    // let mut min_x = i32::MAX;
    // let mut max_x = i32::MIN;

    // sensors.iter().for_each(|s| {
    //     let distance = s.position.distance(&s.closest_beacon);
    //     min_x = min_x.min(s.position.x - distance);
    //     max_x = max_x.max(s.position.x + distance);
    // });
    // let mut not_here: HashSet<i32> = HashSet::new();
    // let y = 10;
    //
    // for x in min_x..=max_x {
    //     let p = Point { x, y };
    //     for sensor in sensors.iter() {
    //         if sensor.position.distance(&p) <= sensor.position.distance(&sensor.closest_beacon) && !beacons.contains(&p) {
    //             not_here.insert(x);
    //             break;
    //         }
    //     }
    // }
    // println!("{}", not_here.len());

    let boxes = sensors.iter().map(|s| s.to_box()).collect::<Vec<Box>>();


    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;

    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    boxes.iter().for_each(|b| {
        min_x = min_x.min(b.top_left.x);
        min_y = min_y.min(b.top_left.y);
        max_x = max_x.max(b.bottom_right.x);
        max_y = max_y.max(b.bottom_right.y);
    });

    boxes.iter().for_each(|b| println!("{:?}", b));

    for x in min_x..=max_x {
        let mut intervals: Vec<(i32, i32)> = Vec::new();
        boxes.iter().for_each(|b| {
            if x < b.top_left.x || x > b.bottom_right.x {
                return;
            }
            let new_interval = (b.top_left.y, b.bottom_right.y);
            let new_intervals = merge_interval(&intervals, new_interval);
            intervals = new_intervals;
        });

        if intervals_has_gaps(&intervals) {
            println!("{} {:?}", x, intervals);
        }
    }

    println!("{:?}", WeirdPoint { x: 6542365, y: 258691 }.to_point());

    println!("{}", 4000000u64 * 3141837u64 + 3400528u64);


    // let mut not_here: HashSet<Point> = HashSet::new();
    //
    // sensors.iter().for_each(|sensor| {
    //     let distance = sensor.position.distance(&sensor.closest_beacon);
    //     let area = PointAreaIterator::new(sensor.position, distance);
    //
    //     for p in area {
    //         not_here.insert(p);
    //     }
    // });
    //
    // for x in 0..=20 {
    //     for y in 0..=20 {
    //         if !not_here.contains(&Point{x,y}){
    //             println!("{} {}", x, y);
    //         }
    //     }
    // }

    // println!("{}", not_here.len());
}
