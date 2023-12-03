use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use crate::State::{ReadingNumber, Space};

struct Number {
    start: usize,
    acc: u32,
}

enum State {
    Space,
    ReadingNumber(Number)
}

#[derive(Eq, Hash, PartialEq)]
struct Coords {
    x: usize,
    y: usize
}

struct Gear {
    numbers: usize,
    sum: u32,
}

fn adjacent_gear(map: &Vec<Vec<char>>, x: usize, y: usize, len: usize) -> Option<Coords> {
    let ystart = 0.max(y as isize -1) as usize;
    let yend = map.len().min(y+2);
    for yy in ystart..yend {
        let xstart = 0.max(x as isize - 1) as usize;
        let xend = map[yy].len().min(x+len+1);
        for xx in xstart..xend {
            let c = map[yy][xx];
            if c == '*' {
                return Some(Coords{x: xx, y: yy})
            }
        }
    }

    None
}

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(f);
    let map = f.lines().map(|x| {
        x.unwrap().chars().collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();

    let mut sum = 0;
    let mut gears:  HashMap<Coords, Gear> = HashMap::new();

    for (y, line) in map.iter().enumerate() {
        let mut state = Space;
        for (x, c) in line.iter().enumerate() {
            match state {
                Space => {
                    if let Some(digit) = c.to_digit(10){
                        state = ReadingNumber(Number {
                            start: x,
                            acc: digit,
                        })
                    }
                }
                ReadingNumber(ref mut number) => {
                    if let Some(digit) = c.to_digit(10) {
                        number.acc *= 10;
                        number.acc += digit;
                    } else {
                        if let Some(gear) = adjacent_gear(&map, number.start, y, x - number.start) {
                            match gears.get_mut(&gear) {
                                None => {
                                    gears.insert(gear, Gear{ numbers: 1, sum: number.acc });
                                }
                                Some(g) => {
                                    g.numbers += 1;
                                    g.sum *= number.acc;
                                }
                            }
                        }
                        state = Space;
                    }
                }
            }
        }

        if let ReadingNumber(ref number) = state {
            if let Some(gear) = adjacent_gear(&map, number.start, y, map[y].len() - number.start - 1) {
                match gears.get_mut(&gear) {
                    None => {
                        gears.insert(gear, Gear{ numbers: 1, sum: number.acc });
                    }
                    Some(g) => {
                        g.numbers += 1;
                        g.sum *= number.acc;
                    }
                }
            }
        }
    }

    for (_, g) in gears.iter() {
        if g.numbers != 2 {
            continue;
        }
        sum += g.sum;
    }

    println!("{sum}");

}
