use std::io::{BufRead, stdin};

use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::opt;
use nom::IResult;
use nom::multi::many1;

fn parse_path_component(input: &str) -> IResult<&str, (u8, Option<char>)> {
    let (input, len) = nom::character::complete::u8(input)?;
    let (input, char) = opt(alt((char('R'), char('L'))))(input)?;

    return Ok((input, (len, char)));
}

fn parse_path(input: &str) -> IResult<&str, Vec<(u8, Option<char>)>> {
    many1(parse_path_component)(input)
}

fn add(a: &(i32, i32), b: &(i32, i32), map: &Vec<String>) -> (i32, i32) {
    ((a.0 + b.0 + map.len() as i32) % map.len() as i32, (a.1 + b.1 + map[0].len() as i32) % map[0].len() as i32)
}

fn add_simple(a: &(i32, i32), b: &(i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn step(cur: &(i32, i32), direction: &(i32, i32), map: &Vec<String>) -> (i32, i32) {
    let mut new = add(cur, direction, map);

    if map[new.0 as usize].chars().nth(new.1 as usize).unwrap() == '#' {
        return *cur;
    }

    while map[new.0 as usize].chars().nth(new.1 as usize).unwrap() != '.' && map[new.0 as usize].chars().nth(new.1 as usize).unwrap() != '#' {
        new = add(&new, direction, map);
    }

    if map[new.0 as usize].chars().nth(new.1 as usize).unwrap() == '#' {
        return *cur;
    }

    new
}

fn mv(cur: &(i32, i32), cur_dir: isize, map: &Vec<String>) -> ((i32, i32), isize) {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let seg = (map.len() / 3) as i32;
    let new = add_simple(cur, &directions[cur_dir as usize]);

    let new_row = new.0;
    let new_col = new.1;

    if cur_dir == 0 {
        if new_col == seg * 3 && new_row / seg == 0 {
            return ((3 * seg - new_row - 1, seg * 4 - 1), 2);
        }
        if new_col == seg * 3 && new_row / seg == 1 {
            return ((2 * seg, 4 * seg - (new_row - seg) - 1), 1);
        }
        if new_col == seg * 4 && new_row / seg == 2 {
            return ((3 * seg - new_row - 1, 3 * seg - 1), 2);
        }
    }

    if cur_dir == 1 {
        if new_row == seg * 2 && new_col / seg == 0 {
            return ((seg * 3 - 1, seg * 3 - new_col - 1), 3);
        }
        if new_row == seg * 2 && new_col / seg == 1 {
            return ((3 * seg - (new_col - seg) - 1, seg * 2), 0);
        }
        if new_row == seg * 3 && new_col / seg == 2 {
            return ((seg * 2 - 1, 3 * seg - new_col - 1), 3);
        }
        if new_row == seg * 3 && new_col / seg == 3 {
            return ((2 * seg - (new_col - 3 * seg) - 1, 0), 0);
        }
    }

    if cur_dir == 2 {
        if new_col == seg * 2 - 1 && new_row / seg == 0 {
            return ((seg, new_row + seg), 1);
        }
        if new_col == -1 && new_row / seg == 1 {
            return ((seg * 3 - 1, 3 * seg + (2 * seg - new_row) - 1), 3);
        }

        if new_col == seg * 2 - 1 && new_row / seg == 2 {
            return ((seg * 2 - 1, seg + (3 * seg - new_row) - 1), 3);
        }
    }

    if cur_dir == 3 {
        if new_row == seg - 1 && new_col / seg == 0 {
            return ((0, 2 * seg + (seg - new_col) - 1), 1);
        }

        if new_row == seg - 1 && new_col / seg == 1 {
            return ((new_col - seg, 2 * seg), 0);
        }

        if new_row == -1 && new_col / seg == 2 {
            return ((seg, seg - (new_col - 2 * seg) - 1), 1);
        }

        if new_row == 2 * seg - 1 && new_col / seg == 3 {
            return ((2 * seg - (new_col - 3 * seg) - 1, 3 * seg - 1), 2);
        }
    }

    (new, cur_dir)
}


fn mv2(cur: &(i32, i32), cur_dir: isize, map: &Vec<String>) -> ((i32, i32), isize) {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let seg = (map.len() / 4) as i32;
    let new = add_simple(cur, &directions[cur_dir as usize]);

    let new_row = new.0;
    let new_col = new.1;

    if cur_dir == 0 {
        if new_col == 3 * seg && new_row / seg == 0 {
            return ((3 * seg - new_row - 1, 2 * seg - 1), 2);
        }
        if new_col == 2 * seg && new_row / seg == 1 {
            return ((seg - 1, new_row - seg + 2 * seg), 3);
        }
        if new_col == 2 * seg && new_row / seg == 2 {
            return (((3 * seg - new_row) - 1, seg * 3 - 1), 2);
        }
        if new_col == 1 * seg && new_row / seg == 3 {
            return ((3 * seg - 1, new_row - 2 * seg), 3);
        }
    }

    if cur_dir == 1 {
        if new_row == 4 * seg && new_col / seg == 0 {
            return ((0, new_col + 2 * seg), 1);
        }
        if new_row == 3 * seg && new_col / seg == 1 {
            return ((new_col + 2 * seg, seg - 1), 2);
        }
        if new_row == 1 * seg && new_col / seg == 2 {
            return ((new_col - seg, 2 * seg - 1), 2);
        }
    }

    if cur_dir == 2 {
        if new_col == 1 * seg - 1 && new_row / seg == 0 {
            return ((3 * seg - new_row - 1, 0), 0);
        }
        if new_col == 1 * seg - 1 && new_row / seg == 1 {
            return ((2 * seg, new_row - seg), 1);
        }
        if new_col == -1 && new_row / seg == 2 {
            return ((3 * seg - new_row - 1, seg), 0);
        }
        if new_col == -1 && new_row / seg == 3 {
            return ((0, new_row - 2 * seg), 1);
        }
    }

    if cur_dir == 3 {
        if new_row == 2 * seg - 1 && new_col / seg == 0 {
            return ((new_col + seg, seg), 0);
        }
        if new_row == -1 && new_col / seg == 1 {
            return ((new_col + 2 * seg, 0), 0);
        }
        if new_row == -1 && new_col / seg == 2 {
            return ((seg * 4 -1, new_col - 2 * seg), 3);
        }
    }

    (new, cur_dir)
}

fn try_mv(cur: &(i32, i32), cur_dir: isize, map: &Vec<String>) -> ((i32, i32), isize) {
    let (new_cur, new_dir) = mv(cur, cur_dir, map);

    if map[new_cur.0 as usize].chars().nth(new_cur.1 as usize).unwrap() == '#' {
        return (*cur, cur_dir);
    }

    (new_cur, new_dir)
}

fn try_mv2(cur: &(i32, i32), cur_dir: isize, map: &Vec<String>) -> ((i32, i32), isize) {
    let (new_cur, new_dir) = mv2(cur, cur_dir, map);

    if map[new_cur.0 as usize].chars().nth(new_cur.1 as usize).unwrap() == '#' {
        return (*cur, cur_dir);
    }

    (new_cur, new_dir)
}

fn main() {
    let lines = stdin().lock().lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let mut it = lines.split(|x| x.len() == 0);
    let map_lines = it.next().unwrap();
    let path_line = it.next().unwrap()[0].clone();

    let max_line = map_lines.iter().map(|l| l.len()).max().unwrap();

    let map = map_lines.iter().map(|row| {
        format!("{: <max$}", row, max = max_line)
    }).collect::<Vec<_>>();

    let path = parse_path(path_line.as_str()).unwrap().1;
    let start = map[0].chars().position(|p| p == '.').unwrap() as i32;
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut cur = (0, start);
    let mut cur_dir = 0isize;
    println!("{} {} {}", cur.0 + 1, cur.1 + 1, cur_dir);

    path.iter().for_each(|(amount, turn)| {
        for _ in 0..*amount {
            cur = step(&cur, &directions[cur_dir as usize], &map);
        }

        if turn.is_some() {
            if turn.unwrap() == 'L' {
                cur_dir = (cur_dir - 1 + directions.len() as isize) % directions.len() as isize
            }

            if turn.unwrap() == 'R' {
                cur_dir = (cur_dir + 1 + directions.len() as isize) % directions.len() as isize
            }
        }
        println!("{} {} {}", cur.0 + 1, cur.1 + 1, cur_dir);
    });
    println!("{}", (cur.0 + 1) * 1000 + (cur.1 + 1) * 4 + cur_dir as i32);

    let mut cur = (0, start);
    let mut cur_dir = 0isize;

    println!("{} {} {}", cur.0 + 1, cur.1 + 1, cur_dir);

    path.iter().for_each(|(amount, turn)| {
        for _ in 0..*amount {
            let (new_cur, new_dir) = try_mv2(&cur, cur_dir, &map);
            cur = new_cur;
            cur_dir = new_dir;
        }


        if turn.is_some() {
            if turn.unwrap() == 'L' {
                cur_dir = (cur_dir - 1 + directions.len() as isize) % directions.len() as isize
            }

            if turn.unwrap() == 'R' {
                cur_dir = (cur_dir + 1 + directions.len() as isize) % directions.len() as isize
            }
        }
        println!("{} {} {}", cur.0 + 1, cur.1 + 1, cur_dir);
    });
    println!("{}", (cur.0 + 1) * 1000 + (cur.1 + 1) * 4 + cur_dir as i32);
}
