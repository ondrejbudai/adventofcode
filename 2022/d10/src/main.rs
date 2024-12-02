use std::io::{BufRead, stdin};

fn main() {
    let mut cycle = 1;
    let mut acc = 1;
    let mut sum = 0;
    stdin().lock().lines().for_each(|l| {
        let line = l.unwrap();

        let instructions = match line.as_str() {
            "noop" => 1,
            _ => 2,
        };

        for _ in 0..instructions {
            if ((cycle - 20) % 40) == 0 {
                sum += cycle * acc;
            }

            if (cycle - 1) % 40 == 0 {
                println!();
            }

            if acc <= cycle - (40 * (cycle / 40)) && acc + 2 >= cycle - (40 * (cycle / 40)) {
                print!("#");
            } else {
                print!(".");
            }

            cycle += 1;
        }

        if line != "noop" {
            let number = line.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
            acc += number;
        }
    });

    println!();
    println!("{}", sum);
}
