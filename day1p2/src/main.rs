use std::fs;

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let digits = digits.map(|t| t.bytes().collect::<Vec<_>>());

    let sum = input.lines().map(|l| {
        let mut first = None;
        let mut last = 0;

        let chars = l.bytes().collect::<Vec<_>>();

        for (i, c) in chars.iter().enumerate(){
            let mut digit = None;
            if *c >= 48 && *c <= 57 {
               digit = Some((*c - 48) as u32);
            } else {
                for (number, d) in digits.iter().enumerate(){
                    if i+d.len() > chars.len() {
                        continue;
                    }
                    if d[..] == chars[i..i+d.len()] {
                        digit = Some((number + 1) as u32);
                    }
                }
            }

            if let Some(digit) = digit {
                last = digit;
                if let None = first {
                    first = Some(digit);
                }
            }

        }

        first.unwrap() * 10 + last
    }).sum::<u32>();

    println!("{sum}");
}
