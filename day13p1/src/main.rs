use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_map() -> Vec<Vec<Vec<char>>> {
    let f = File::open("input").unwrap();
    let lines = BufReader::new(f).lines();

    let mut chunks = Vec::new();
    let mut current_chunk = Vec::new();

    for line in lines {
        let line = line.unwrap();
        if !line.is_empty() {
            current_chunk.push(line.chars().collect());
        } else {
            chunks.push(current_chunk);
            current_chunk = vec![];
        }
    }

    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }

    chunks
}
fn main() {
    let map = read_map();

    let sum: usize = map.iter().map(|m| {
        't:
        for i in 0..m.len()-1 {
            let num = (i + 1).min(m.len() - i - 1);

            for j in 0..num {
                if m[i-j] != m[i+j+1] {
                    continue 't;
                }
            }

            return (i + 1) * 100;
        }

        't:
        for i in 0..m[0].len()-1 {
            let num = (i + 1).min(m[0].len() - i - 1);

            for j in 0..num {
                for y in 0..m.len() {
                    if m[y][i-j] != m[y][i+j+1] {
                        continue 't;
                    }
                }

            }

            return i + 1;
        }

        0
    }).sum();

    println!("{sum}");
}
