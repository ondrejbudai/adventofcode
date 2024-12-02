use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let f = File::open("input").unwrap();
    let mut lines = BufReader::new(f).lines();

    let time: u64 = lines.next().unwrap().unwrap().split_whitespace().skip(1).collect::<Vec<_>>().join("").parse().unwrap();
    let record: u64 = lines.next().unwrap().unwrap().split_whitespace().skip(1).collect::<Vec<_>>().join("").parse().unwrap();
    let product: usize = (1_u64..time).map(|t| {

        t * (time - t)

    }).filter(|t| *t > record).count();

    println!("{product}");
}
