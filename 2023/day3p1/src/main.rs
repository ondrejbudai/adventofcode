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

fn has_adjacent_symbol(map: &Vec<Vec<char>>, x: usize, y: usize, len: usize) -> bool {
    let ystart = 0.max(y as isize -1) as usize;
    let yend = map.len().min(y+2);
    for yy in ystart..yend {
        let xstart = 0.max(x as isize - 1) as usize;
        let xend = map[yy].len().min(x+len+1);
        for xx in xstart..xend {
            let c = map[yy][xx];
            if !c.is_digit(10) && c != '.' {
                return true
            }
        }
    }

    false
}

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(f);
    let map = f.lines().map(|x| {
        x.unwrap().chars().collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();

    let mut sum = 0;

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
                        if has_adjacent_symbol(&map, number.start, y, x - number.start) {
                            sum += number.acc;
                        }
                        state = Space;
                    }
                }
            }
        }

        if let ReadingNumber(ref number) = state {
            if has_adjacent_symbol(&map, number.start, y, map[y].len() - number.start - 1) {
                sum += number.acc;
            }
        }
    }

    println!("{sum}");

}
