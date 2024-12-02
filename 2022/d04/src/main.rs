use regex::Regex;

fn main() {
    use std::io::{self, BufRead};
    let stdin = io::stdin();

    let mut overlaps = 0;
    let mut overlaps2 = 0;

    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    stdin.lock().lines().for_each(|line_wrapped| {
        let line = line_wrapped.unwrap();

        let captures = re.captures(line.as_str()).unwrap();

        let elf1_start = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let elf1_end = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let elf2_start = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let elf2_end = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();

        if (elf2_start >= elf1_start && elf2_end <= elf1_end) || (elf1_start >= elf2_start && elf1_end <= elf2_end) {
            overlaps += 1;
        }

        if (elf2_start <= elf1_end && elf2_end >= elf1_start) || (elf1_start <= elf2_end && elf1_end >= elf2_start) {
            overlaps2 += 1;
        }
    });
    println!("{} {}", overlaps, overlaps2)
}
