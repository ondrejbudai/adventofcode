use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let sum = input.lines().map(|l| {
        let mut first = None;
        let mut last = 0;

        l.chars().for_each(|c| {
            let digit = c.to_digit(10);
            if let Some(digit) = digit {
                last = digit;
                if let None = first {
                    first = Some(digit);
                }
            }
        });

        first.unwrap() * 10 + last
    }).sum::<u32>();

    println!("{sum}");
}
