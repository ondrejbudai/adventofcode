use std::cell::{Ref, RefCell};
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{BufRead, stdin};
use std::rc::Rc;

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::combinator::eof;
use nom::IResult;
use nom::multi::separated_list1;

fn dijkstra(map: &HashMap<String, Valve>, start: &str, end: &str) -> Option<usize> {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut to_visit: VecDeque<(&str, usize)> = VecDeque::new();
    to_visit.push_back((start, 0));
    visited.insert(start);

    loop {
        if to_visit.len() == 0 {
            return None;
        }
        let cur = to_visit.pop_front().unwrap();
        if cur.0 == end {
            return Some(cur.1);
        }

        map.get(cur.0).unwrap().tunnels.iter().for_each(|name| {
            if !visited.contains(name.as_str()) {
                to_visit.push_back((name, cur.1 + 1));
                visited.insert(name);
            }
        });
    }
}


#[derive(Debug)]
struct Valve {
    flow_rate: u16,
    tunnels: Vec<String>,
}

fn parse_valve(input: &str) -> IResult<&str, (String, Valve)> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, name) = alphanumeric1(input)?;
    let (input, _) = tag(" has flow rate=")(input)?;
    let (input, flow_rate) = nom::character::complete::u16(input)?;
    let (input, _) = alt((tag("; tunnel leads to valve "), tag("; tunnels lead to valves ")))(input)?;
    let (input, tunnels) = separated_list1(tag(", "), alphanumeric1)(input)?;

    eof(input)?;

    Ok((input, (name.to_string(), Valve {
        flow_rate,
        tunnels: tunnels.iter().map(|x| { x.to_string() }).collect(),
    })))
}

fn step(last_valve: usize, remaining_time: usize, distances: &HashMap<(usize, usize), usize>, to_visit: &HashSet<usize>, sum: usize, rate: usize, rates: &HashMap<usize, u16>, cache: &RefCell<HashMap<Vec<usize>, usize>>, is_el: bool) -> usize {
    let mut best = sum;
    if to_visit.len() == 0 {
        return sum + remaining_time * rate;
    }
    for valve in to_visit.iter() {
        let distance = distances.get(&(last_valve, *valve)).unwrap() + 1;
        if distance > remaining_time {
            let mut elephant = 0;
            if !is_el {
                let mut blah = to_visit.iter().map(|a| *a).collect::<Vec<usize>>();
                blah.sort();

                let mut cache_worked = false;
                {
                    let c = cache.borrow();
                    let maybe_cached_value = c.get(&blah);
                    if !maybe_cached_value.is_none() {
                        elephant = *maybe_cached_value.unwrap();
                        cache_worked = true;
                    }
                }

                if !cache_worked {
                    elephant = step(0, 26, distances, to_visit, 0, 0, rates, cache, true);
                    cache.borrow_mut().insert(blah, elephant);
                }
            }

            let res = sum + elephant + remaining_time * rate;
            best = best.max(res);
            continue;
        }
        let mut to_visit = to_visit.clone();
        to_visit.remove(&valve);
        let sum = sum + distance * rate;
        let rate = rate + *rates.get(valve).unwrap() as usize;

        let res = step(*valve, remaining_time - distance, distances, &to_visit, sum, rate, rates, cache, is_el);
        best = best.max(res);
    }

    best
}


fn main() {
    let valves = stdin().lock().lines().map(|x| {
        parse_valve(x.unwrap().as_str()).unwrap().1
    }).collect::<HashMap<String, Valve>>();

    let mut flow_valves_names = vec!["AA"];

    valves.iter().for_each(|(name, v)| {
        if v.flow_rate > 0 {
            flow_valves_names.push(name.as_str());
        }
    });

    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();

    let mut min_distance = 100;

    flow_valves_names.iter().enumerate().for_each(|(ai, a)| {
        flow_valves_names.iter().enumerate().for_each(|(bi, b)| {
            let d = dijkstra(&valves, *a, *b).unwrap();
            if d < min_distance && d > 0 {
                min_distance = d;
            }
            distances.insert((ai, bi), d);
        });
    });

    let rates = flow_valves_names.iter().enumerate().skip(1).map(|(i, valve_name)| {
        (i, valves.get(*valve_name).unwrap().flow_rate)
    }).collect::<HashMap<usize, u16>>();

    let to_visit = (1..flow_valves_names.len()).collect::<HashSet<usize>>();

    let cache = RefCell::new(HashMap::new());

    let result = step(0, 26, &distances, &to_visit, 0, 0, &rates, &cache, false);

    println!("{result}");

    // println!("{min_distance}");
    //
    // let mut max_flow = 0;
    // let mut max_path = Vec::new();
    //
    // let mut counter = 0u64;
    //
    // for names in flow_valves_names.iter().permutations(8) {
    //     counter += 1;
    //     if counter % 100000000 == 0 {
    //         println!("{counter}");
    //     }
    //     if **names.get(0).unwrap() != "AA" {
    //         continue;
    //     }
    //     let mut sum = 0;
    //     let mut rate = 0;
    //     let mut time = 0;
    //     for (start_i, end_i) in (0..names.len() - 1).zip(1..names.len()) {
    //         let start = **names.get(start_i).unwrap();
    //         let end = **names.get(end_i).unwrap();
    //
    //         let distance = distances.get(&(start, end)).unwrap() + 1;
    //         if time + distance > 30 {
    //             break;
    //         }
    //         time += distance;
    //         sum += distance * rate;
    //         rate += valves.get(end).unwrap().flow_rate as usize;
    //     }
    //
    //     sum += rate * (30 - time);
    //
    //     if sum > max_flow {
    //         max_flow = sum;
    //         max_path = names;
    //         println!("{}, {:?}", max_flow, max_path);
    //     }
    //
    //     max_flow = max_flow.max(sum);
    // }
}
