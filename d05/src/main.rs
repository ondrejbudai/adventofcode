use std::collections::LinkedList;
use std::io::{BufRead, stdin};

fn main() {
    let mut parsing_initial_state = true;
    let mut stacks9000: [LinkedList<char>; 9] = Default::default();
    let mut stacks9001: [LinkedList<char>; 9] = Default::default();

    stdin().lock().lines().for_each(|line_raw| {
        let line = line_raw.unwrap();

        if line == "" {
            parsing_initial_state = false;
            return;
        }

        if line.starts_with(" 1 ") {
            return;
        }

        if parsing_initial_state {
            line.as_bytes()
                .chunks(4)
                .map(|buf| unsafe { std::str::from_utf8_unchecked(buf) })
                .enumerate().for_each(|(i, crte)| {
                    if crte.chars().nth(1).unwrap() == ' ' {
                        return;
                    }
                    stacks9000[i].push_back(crte.chars().nth(1).unwrap());
                    stacks9001[i].push_back(crte.chars().nth(1).unwrap());
                });
            return;
        }

        let splitted = line.split(" ").collect::<Vec<&str>>();

        let amount = splitted.get(1).unwrap().parse::<usize>().unwrap();
        let from = splitted.get(3).unwrap().parse::<usize>().unwrap();
        let to = splitted.get(5).unwrap().parse::<usize>().unwrap();

        for _ in 0..amount {
            let crt = stacks9000[from-1].pop_front().unwrap();
            stacks9000[to-1].push_front(crt);
        }

        let rest = stacks9001[from-1].split_off(amount);
        for _ in 0..amount {
            let crt = stacks9001[from-1].pop_back().unwrap();
            stacks9001[to-1].push_front(crt);
        }

        stacks9001[from-1] = rest;
    });

    stacks9000.iter().for_each(|stack| {
        print!("{}", stack.front().unwrap())
    });
    println!();
    stacks9001.iter().for_each(|stack| {
        print!("{}", stack.front().unwrap())
    });
}
