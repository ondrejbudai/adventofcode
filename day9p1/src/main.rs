use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(f);
    let mut lines = f.lines();

    let sum: i32 = lines.map(|line| {
        let line = line.unwrap();

        let mut nums: Vec<i32> = line.split(" ").map(|num| num.parse().unwrap()).collect();
        let mut lasts = vec![nums.last().unwrap().clone()];


        loop {
            let mut new_nums: Vec<i32> = nums.windows(2).map(|x| x[1] - x[0]).collect();

            nums = new_nums;

            lasts.push(nums.last().unwrap().clone());


            if nums.iter().all(|x| *x == 0) {
                break
            }
        }

        lasts.iter().fold(0, |acc, c| acc+ c)
    }).sum();

    println!("{sum}");
}
