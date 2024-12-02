use std::fs;

fn hash(s: &str) -> u32 {
    let mut cur = 0u32;

    s.bytes().for_each(|x| {
        cur += x as u32;
        cur *= 17;
        cur %= 256;
    });

    cur
}

fn main() {
    let s = fs::read_to_string("input").unwrap();
    let sum: u32 = s.trim().split(",").map(hash).sum();
    println!("{sum}");
}
