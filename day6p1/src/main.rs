use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let f = File::open("input").unwrap();
    let mut lines = BufReader::new(f).lines();

    let times: Vec<u32> = lines.next().unwrap().unwrap().split_whitespace().skip(1).map(|x| x.parse().unwrap()).collect();
    let records: Vec<u32> = lines.next().unwrap().unwrap().split_whitespace().skip(1).map(|x| x.parse().unwrap()).collect();

    let product: usize = times.iter().zip(records).map(|(time, record)| {
        (1_u32..*time).map(|t| {

            t * (*time - t)

        }).filter(|t| *t > record).count()
    }).product();

    println!("{product}");
}
