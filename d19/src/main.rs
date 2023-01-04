use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, stdin};

use nom::bytes::complete::tag;
use nom::combinator::eof;
use nom::IResult;

struct Blueprint {
    ore_robot_ore_cost: u16,
    clay_robot_ore_cost: u16,
    obsidian_robot_ore_cost: u16,
    obsidian_robot_clay_cost: u16,
    geode_robot_ore_cost: u16,
    geode_robot_obsidian_cost: u16,
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, _) = tag("Blueprint ")(input)?;
    let (input, _) = nom::character::complete::u16(input)?;
    let (input, _) = tag(": Each ore robot costs ")(input)?;
    let (input, ore_robot_ore_cost) = nom::character::complete::u16(input)?;
    let (input, _) = tag(" ore. Each clay robot costs ")(input)?;
    let (input, clay_robot_ore_cost) = nom::character::complete::u16(input)?;
    let (input, _) = tag(" ore. Each obsidian robot costs ")(input)?;
    let (input, obsidian_robot_ore_cost) = nom::character::complete::u16(input)?;
    let (input, _) = tag(" ore and ")(input)?;
    let (input, obsidian_robot_clay_cost) = nom::character::complete::u16(input)?;
    let (input, _) = tag(" clay. Each geode robot costs ")(input)?;
    let (input, geode_robot_ore_cost) = nom::character::complete::u16(input)?;
    let (input, _) = tag(" ore and ")(input)?;
    let (input, geode_robot_obsidian_cost) = nom::character::complete::u16(input)?;
    let (input, _) = tag(" obsidian.")(input)?;

    let (input, _) = eof(input)?;

    Ok((input, Blueprint {
        ore_robot_ore_cost,
        clay_robot_ore_cost,
        obsidian_robot_ore_cost,
        obsidian_robot_clay_cost,
        geode_robot_ore_cost,
        geode_robot_obsidian_cost,
    }))
}

#[derive(Hash, Clone, PartialEq, Eq)]
struct State {
    ore: u16,
    clay: u16,
    obsidian: u16,
    ore_robots: u16,
    clay_robots: u16,
    obsidian_robots: u16,
    geode_robots: u16,
    minute: u16,
    geodes: u16,
}

impl State {
    fn update(&mut self, old: &Self) {
        self.ore += old.ore_robots;
        self.clay += old.clay_robots;
        self.obsidian += old.obsidian_robots;
        self.geodes += old.geode_robots;
        self.minute += 1;
    }
}

fn simulate(bp: &Blueprint, time: u16) -> u16 {
    let mut cache: HashSet<State> = HashSet::<State>::new();
    let mut to_visit = Vec::<State>::new();

    let start = State {
        ore: 0,
        clay: 0,
        obsidian: 0,
        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        geode_robots: 0,
        minute: 1,
        geodes: 0,
    };

    to_visit.push(start.clone());
    cache.insert(start);

    let mut max_geodes = 0;

    while to_visit.len() > 0 {
        let state = to_visit.pop().unwrap();

        if state.minute == time + 1 {
            if state.geodes > max_geodes {
                max_geodes = state.geodes;
                println!("{}", max_geodes);
            }
            continue;
        }

        let remaining_time = time - state.minute;
        let prediction = state.geodes + state.geode_robots + (time - state.minute) * state.geode_robots + ((remaining_time * remaining_time + remaining_time) / 2);
        if prediction <= max_geodes {
            continue;
        }


        let mut can_buy_everything = true;
        if state.ore >= bp.ore_robot_ore_cost {
            let mut new_state = state.clone();
            new_state.ore -= bp.ore_robot_ore_cost;
            new_state.ore_robots += 1;
            new_state.update(&state);
            if !cache.contains(&new_state) {
                to_visit.push(new_state.clone());
                cache.insert(new_state);
            }
        } else {
            can_buy_everything = false;
        }

        if state.ore >= bp.clay_robot_ore_cost {
            let mut new_state = state.clone();
            new_state.ore -= bp.clay_robot_ore_cost;
            new_state.clay_robots += 1;
            new_state.update(&state);
            if !cache.contains(&new_state) {
                to_visit.push(new_state.clone());
                cache.insert(new_state);
            }
        } else {
            can_buy_everything = false;
        }

        if state.ore >= bp.obsidian_robot_ore_cost && state.clay >= bp.obsidian_robot_clay_cost {
            let mut new_state = state.clone();
            new_state.ore -= bp.obsidian_robot_ore_cost;
            new_state.clay -= bp.obsidian_robot_clay_cost;
            new_state.obsidian_robots += 1;
            new_state.update(&state);
            if !cache.contains(&new_state) {
                to_visit.push(new_state.clone());
                cache.insert(new_state);
            }
        } else {
            can_buy_everything = false;
        }

        if state.ore >= bp.geode_robot_ore_cost && state.obsidian >= bp.geode_robot_obsidian_cost {
            let mut new_state = state.clone();
            new_state.ore -= bp.geode_robot_ore_cost;
            new_state.obsidian -= bp.geode_robot_obsidian_cost;
            new_state.geode_robots += 1;
            new_state.update(&state);
            if !cache.contains(&new_state) {
                to_visit.push(new_state.clone());
                cache.insert(new_state);
            }
        } else {
            can_buy_everything = false;
        }

        if !can_buy_everything {
            let mut new_state = state.clone();
            new_state.update(&state);
            if !cache.contains(&new_state) {
                to_visit.push(new_state.clone());
                cache.insert(new_state);
            }
        }
    }

    max_geodes
}

fn main() {
    let blueprints = stdin().lock().lines().map(|line| {
        parse_blueprint(line.unwrap().as_str()).unwrap().1
    }).collect::<Vec<Blueprint>>();
    println!("{}", blueprints.iter().map(|bp| {
        simulate(bp, 24)
    }).enumerate().map(|(i, best)| (i + 1) * best as usize).sum::<usize>());
    println!("{}", blueprints.iter().take(3).map(|bp| {
        simulate(bp, 32)
    }).fold(1u64, |product, val| product * val as u64));
}
