use std::io::{Read, stdin};

const LEFT: u16 = '<' as u16;
const RIGHT: u16 = '>' as u16;

type Map = Vec<[bool; 7]>;
type Coords = (isize, isize);
type Shape = &'static [&'static [bool]];

fn is_occupied(map: &Map, coords: Coords) -> bool {
    if coords.0 < 0 || coords.1 < 0 || coords.1 > 6 {
        return true;
    }

    if coords.0 as usize >= map.len() {
        return false;
    }

    return map.get(coords.0 as usize).unwrap()[coords.1 as usize];
}

fn can_be_placed(map: &Map, coords: Coords, shape: Shape) -> bool {
    for (shape_row, row) in shape.iter().enumerate() {
        for (shape_col, occupied) in row.iter().enumerate() {
            if !occupied {
                continue;
            }

            if is_occupied(map, (coords.0 + shape_row as isize, coords.1 + shape_col as isize)) {
                return false;
            }
        }
    }

    true
}

fn place(map: &mut Map, coords: Coords, shape: Shape) {
    let missing_rows = coords.0 + shape.len() as isize - map.len() as isize;

    for _ in 0..missing_rows {
        map.push(Default::default());
    }
    for (shape_row, row) in shape.iter().enumerate() {
        for (shape_col, occupied) in row.iter().enumerate() {
            if !occupied {
                continue;
            }
            map[coords.0 as usize + shape_row][coords.1 as usize + shape_col] = true;
        }
    }
}

fn show(map: &Map, coords: Coords, shape: Shape) {
    let max = map.len().max(coords.0 as usize + shape.len());
    (0..max).rev().for_each(|row| {
        (0..7).for_each(|col| {
            if is_occupied(map, (row as isize, col as isize)) {
                print!("#");
                return;
            }

            let shape_row = row as isize - coords.0;
            let shape_col = col as isize - coords.1;

            if shape_row >= 0 && shape_row < shape.len() as isize && shape_col >= 0 && shape_col < shape[0].len() as isize && shape[shape_row as usize][shape_col as usize] {
                print!("@");
                return;
            }

            print!(".");

        });
        println!();
    });
    println!();
}

fn main() {
    let rollover_count = (1000000000000u64 - 1682) / 1690;
    let rollover_rest = (1000000000000u64 - 1682) % 1690;

    println!("{}", rollover_count * 2548 + 2537 + 3390 - 2537);
    let mut map: Map = Vec::new();
    let shapes: [Shape; 5] = [
        &[&[true, true, true, true]],
        &[&[false, true, false], &[true, true, true], &[false, true, false]],
        &[&[true, true, true], &[false, false, true], &[false, false, true]],
        &[&[true], &[true], &[true], &[true]],
        &[&[true, true], &[true, true]],
    ];
    let mut next_shape = 1usize;

    let winds = stdin().lock().bytes().map(|x| x.unwrap()).filter(|x1| *x1 != '\n' as u16).collect::<Vec<u16>>();
    let mut next_wind = 0usize;
    let mut rocks_stopped = 0usize;

    // row, col
    let mut cur_location: Coords = (3isize, 2isize);
    let mut cur_shape = shapes[0];
    loop {
        let wind = winds[next_wind];

        // next_wind = (next_wind + 1) % winds.len();
        next_wind += 1;
        if next_wind == winds.len() {
            println!("{}, {}", rocks_stopped, map.len());
            next_wind = 0;
        }

        if wind == LEFT {
            let new_location = (cur_location.0, cur_location.1 - 1);
            if can_be_placed(&map, new_location, cur_shape) {
                cur_location = new_location;
            }
        } else if wind == RIGHT {
            let new_location = (cur_location.0, cur_location.1 + 1);
            if can_be_placed(&map, new_location, cur_shape) {
                cur_location = new_location;
            }
        } else {
            panic!("wtf");
        }

        let new_location = (cur_location.0 - 1, cur_location.1);
        if !can_be_placed(&map, new_location, cur_shape) {
            place(&mut map, cur_location, cur_shape);
            rocks_stopped += 1;
            if rocks_stopped  == rollover_rest as usize + 1682 {
                println!("{}, {}", rocks_stopped, map.len());
            }
            if rocks_stopped == 1000000000000 {
                println!("{}", map.len());
                break;
            }
            cur_location = (map.len() as isize + 3, 2isize);
            cur_shape = shapes[next_shape];
            // show(&map, cur_location, cur_shape);
            next_shape = (next_shape + 1) % shapes.len();
            continue;
        }
        cur_location = new_location;
        // show(&map, cur_location, cur_shape);
    }
}
