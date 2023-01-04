fn main() {
    use std::io::{self, BufRead};
    let stdin = io::stdin();

    let mut elves = vec![0];
    for rawline in stdin.lock().lines() {
        let line = rawline.unwrap();

        if line.is_empty() {
            elves.push(0);
            continue;
        }

        let value = line.parse::<i32>().unwrap();
        *elves.last_mut().unwrap() += value;
    }

    elves.sort();
    elves.reverse();

    println!("{}", elves[0] + elves[1] + elves[2])
}
