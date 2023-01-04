use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{BufRead, stdin};
use std::ops::{Add, AddAssign};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Coords(i32, i32);

impl AddAssign for Coords {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Add for Coords {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coords(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn print(elves: &HashMap<Coords, Coords>) {
    let mut min_row = i32::MAX;
    let mut max_row = i32::MIN;
    let mut min_col = i32::MAX;
    let mut max_col = i32::MIN;

    elves.iter().for_each(|(e, _)| {
        min_row = min_row.min(e.0);
        max_row = max_row.max(e.0);
        min_col = min_col.min(e.1);
        max_col = max_col.max(e.1);
    });

    for row in min_row..=max_row {
        for col in min_col..=max_col {
            if elves.contains_key(&Coords(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }
    println!();
}

fn main() {
    let mut elves = HashMap::new();

    stdin().lock().lines().enumerate().for_each(|(row, line)| {
        line.unwrap().chars().enumerate().for_each(|(col, c)| {
            if c == '#' {
                elves.insert(Coords(row as i32, col as i32), Coords(row as i32, col as i32));
            }
        });
    });

    let mut moves = VecDeque::new();
    moves.push_back([Coords(-1, 0), Coords(-1, 1), Coords(-1, -1)]);
    moves.push_back([Coords(1, 0), Coords(1, 1), Coords(1, -1)]);
    moves.push_back([Coords(0, -1), Coords(-1, -1), Coords(1, -1)]);
    moves.push_back([Coords(0, 1), Coords(-1, 1), Coords(1, 1)]);

    let mut elves_cur = elves.clone();

    for _ in 0..10 {
        let mut revert = HashSet::new();
        let mut new = HashMap::new();

        elves_cur.iter().for_each(|(e, _)| {
            let mut new_pos_found = false;

            if moves.iter().all(|m| m.iter().all(|m| !elves_cur.contains_key(&(*e + *m)))) {
                new.insert(*e, *e);
                return;
            }
            for m in moves.iter() {
                let all_free = m.iter().all(|m| {
                    !elves_cur.contains_key(&(*e + *m))
                });

                if all_free {
                    let new_dest = *e + m[0];
                    if new.contains_key(&new_dest) {
                        revert.insert(new_dest);
                    } else {
                        new.insert(new_dest, *e);
                        new_pos_found = true;
                    }
                    break;
                }
            }

            if !new_pos_found {
                new.insert(*e, *e);
            }
        });

        revert.iter().for_each(|x| {
            let old_pos = new[x];
            new.remove(x);
            new.insert(old_pos, old_pos);
        });


        let tmp = moves.pop_front().unwrap();
        moves.push_back(tmp);

        elves_cur = new;
    }

    let mut min_row = i32::MAX;
    let mut max_row = i32::MIN;
    let mut min_col = i32::MAX;
    let mut max_col = i32::MIN;

    elves.iter().for_each(|(e, _)| {
        min_row = min_row.min(e.0);
        max_row = max_row.max(e.0);
        min_col = min_col.min(e.1);
        max_col = max_col.max(e.1);
    });

    println!("{}", (max_row - min_row + 1) * (max_col - min_col + 1) - elves.len() as i32);


    let mut moves = VecDeque::new();
    moves.push_back([Coords(-1, 0), Coords(-1, 1), Coords(-1, -1)]);
    moves.push_back([Coords(1, 0), Coords(1, 1), Coords(1, -1)]);
    moves.push_back([Coords(0, -1), Coords(-1, -1), Coords(1, -1)]);
    moves.push_back([Coords(0, 1), Coords(-1, 1), Coords(1, 1)]);
    let mut elves_cur = elves.clone();

    let mut round = 0;

    loop {
        let mut revert = HashSet::new();
        let mut new = HashMap::new();

        elves_cur.iter().for_each(|(e, _)| {
            let mut new_pos_found = false;

            if moves.iter().all(|m| m.iter().all(|m| !elves_cur.contains_key(&(*e + *m)))) {
                new.insert(*e, *e);
                return;
            }
            for m in moves.iter() {
                let all_free = m.iter().all(|m| {
                    !elves_cur.contains_key(&(*e + *m))
                });

                if all_free {
                    let new_dest = *e + m[0];
                    if new.contains_key(&new_dest) {
                        revert.insert(new_dest);
                    } else {
                        new.insert(new_dest, *e);
                        new_pos_found = true;
                    }
                    break;
                }
            }

            if !new_pos_found {
                new.insert(*e, *e);
            }
        });

        revert.iter().for_each(|x| {
            let old_pos = new[x];
            new.remove(x);
            new.insert(old_pos, old_pos);
        });


        let tmp = moves.pop_front().unwrap();
        moves.push_back(tmp);

        if elves_cur == new {
            println!("{round}");
            break;
        }
        elves_cur = new;

        round += 1;
    }
}
