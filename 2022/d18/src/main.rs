use std::collections::{HashSet, VecDeque};
use std::io::{BufRead, stdin};

fn dijkstra(map: &HashSet<(i8, i8, i8)>, start: &(i8, i8, i8), end: &(i8, i8, i8)) -> (bool, HashSet<(i8, i8, i8)>) {
    let mut visited: HashSet<(i8, i8, i8)> = HashSet::new();
    let mut to_visit: VecDeque<(i8, i8, i8)> = VecDeque::new();
    to_visit.push_back(*start);
    visited.insert(*start);

    loop {
        if to_visit.len() == 0 {
            return (false, visited);
        }
        let cur = to_visit.pop_front().unwrap();
        if cur == *end {
            return (true, visited);
        }
        if !visited.contains(&(cur.0 - 1, cur.1, cur.2)) && !map.contains(&(cur.0 - 1, cur.1, cur.2)) {
            to_visit.push_back((cur.0 - 1, cur.1, cur.2));
            visited.insert((cur.0 - 1, cur.1, cur.2));
        }
        if !visited.contains(&(cur.0 + 1, cur.1, cur.2)) && !map.contains(&(cur.0 + 1, cur.1, cur.2)) {
            to_visit.push_back((cur.0 + 1, cur.1, cur.2));
            visited.insert((cur.0 + 1, cur.1, cur.2));
        }
        if !visited.contains(&(cur.0, cur.1 - 1, cur.2)) && !map.contains(&(cur.0, cur.1 - 1, cur.2)) {
            to_visit.push_back((cur.0, cur.1 - 1, cur.2));
            visited.insert((cur.0, cur.1 - 1, cur.2));
        }
        if !visited.contains(&(cur.0, cur.1 + 1, cur.2)) && !map.contains(&(cur.0, cur.1 + 1, cur.2)) {
            to_visit.push_back((cur.0, cur.1 + 1, cur.2));
            visited.insert((cur.0, cur.1 + 1, cur.2));
        }
        if !visited.contains(&(cur.0, cur.1, cur.2 - 1)) && !map.contains(&(cur.0, cur.1, cur.2 - 1)) {
            to_visit.push_back((cur.0, cur.1, cur.2 - 1));
            visited.insert((cur.0, cur.1, cur.2 - 1));
        }
        if !visited.contains(&(cur.0, cur.1, cur.2 + 1)) && !map.contains(&(cur.0, cur.1, cur.2 + 1)) {
            to_visit.push_back((cur.0, cur.1, cur.2 + 1));
            visited.insert((cur.0, cur.1, cur.2 + 1));
        }
    }
}


fn main() {
    let rocks: HashSet<(i8, i8, i8)> = stdin().lock().lines().map(|x| {
        let line = x.unwrap();
        let mut it = line.split(",");
        (
            it.next().unwrap().parse::<i8>().unwrap(),
            it.next().unwrap().parse::<i8>().unwrap(),
            it.next().unwrap().parse::<i8>().unwrap(),
        )
    }).collect();

    let mut open_sides = 0;

    rocks.iter().for_each(|rock| {
        if !rocks.contains(&(rock.0 - 1, rock.1, rock.2)) {
            open_sides += 1;
        }
        if !rocks.contains(&(rock.0, rock.1 - 1, rock.2)) {
            open_sides += 1;
        }
        if !rocks.contains(&(rock.0, rock.1, rock.2 - 1)) {
            open_sides += 1;
        }
        if !rocks.contains(&(rock.0 + 1, rock.1, rock.2)) {
            open_sides += 1;
        }
        if !rocks.contains(&(rock.0, rock.1 + 1, rock.2)) {
            open_sides += 1;
        }
        if !rocks.contains(&(rock.0, rock.1, rock.2 + 1)) {
            open_sides += 1;
        }
    });

    println!("{}", open_sides);

    let mut open_sides = 0;

    let mut proccessed = 0;

    println!("rocks: {}", rocks.len());

    let mut outside: HashSet<(i8, i8, i8)> = HashSet::new();
    let mut inside: HashSet<(i8, i8, i8)> = HashSet::new();

    rocks.iter().for_each(|rock| {
        if proccessed % 100 == 0 {
            println!("{proccessed}");
        }
        proccessed += 1;

        if outside.contains(&(rock.0 - 1, rock.1, rock.2)) {
            open_sides += 1;
        } else if !inside.contains(&(rock.0 - 1, rock.1, rock.2)) &&  !rocks.contains(&(rock.0 - 1, rock.1, rock.2)) {
            let (is_outside, set) = dijkstra(&rocks, &(rock.0 - 1, rock.1, rock.2), &(-1, -1, -1));
            if is_outside {
                open_sides += 1;
                outside.extend(set.iter());
            } else {
                inside.extend(set.iter());
            }
        }

        if outside.contains(&(rock.0 + 1, rock.1, rock.2)) {
            open_sides += 1;
        } else if !inside.contains(&(rock.0 + 1, rock.1, rock.2)) &&  !rocks.contains(&(rock.0 + 1, rock.1, rock.2)) {
            let (is_outside, set) = dijkstra(&rocks, &(rock.0 + 1, rock.1, rock.2), &(-1, -1, -1));
            if is_outside {
                open_sides += 1;
                outside.extend(set.iter());
            } else {
                inside.extend(set.iter());
            }
        }

        if outside.contains(&(rock.0, rock.1 - 1, rock.2)) {
            open_sides += 1;
        } else if !inside.contains(&(rock.0, rock.1 - 1, rock.2)) &&  !rocks.contains(&(rock.0, rock.1 - 1, rock.2)) {
            let (is_outside, set) = dijkstra(&rocks, &(rock.0, rock.1 - 1, rock.2), &(-1, -1, -1));
            if is_outside {
                open_sides += 1;
                outside.extend(set.iter());
            } else {
                inside.extend(set.iter());
            }
        }

        if outside.contains(&(rock.0, rock.1 + 1, rock.2)) {
            open_sides += 1;
        } else if !inside.contains(&(rock.0, rock.1 + 1, rock.2)) &&  !rocks.contains(&(rock.0, rock.1 + 1, rock.2)) {
            let (is_outside, set) = dijkstra(&rocks, &(rock.0, rock.1 + 1, rock.2), &(-1, -1, -1));
            if is_outside {
                open_sides += 1;
                outside.extend(set.iter());
            } else {
                inside.extend(set.iter());
            }
        }

        if outside.contains(&(rock.0, rock.1, rock.2 - 1)) {
            open_sides += 1;
        } else if !inside.contains(&(rock.0, rock.1, rock.2 - 1)) &&  !rocks.contains(&(rock.0, rock.1, rock.2 - 1)) {
            let (is_outside, set) = dijkstra(&rocks, &(rock.0, rock.1, rock.2 - 1), &(-1, -1, -1));
            if is_outside {
                open_sides += 1;
                outside.extend(set.iter());
            } else {
                inside.extend(set.iter());
            }
        }

        if outside.contains(&(rock.0, rock.1, rock.2 + 1)) {
            open_sides += 1;
        } else if !inside.contains(&(rock.0, rock.1, rock.2 + 1)) &&  !rocks.contains(&(rock.0, rock.1, rock.2 + 1)) {
            let (is_outside, set) = dijkstra(&rocks, &(rock.0, rock.1, rock.2 + 1), &(-1, -1, -1));
            if is_outside {
                open_sides += 1;
                outside.extend(set.iter());
            } else {
                inside.extend(set.iter());
            }
        }
    });

    println!("{}", open_sides);
}
