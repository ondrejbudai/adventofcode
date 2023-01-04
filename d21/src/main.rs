use std::collections::HashMap;
use std::io::{BufRead, stdin};

fn step(monkey: &str, input: &HashMap<String, String>, cache: &mut HashMap<String, i64>) {
    let right_side = &input[monkey];

    if let Ok(number) = right_side.parse::<i64>() {
        cache.insert(monkey.to_string(), number);
        return;
    }

    let m1 = &right_side[0..4];
    let m2 = &right_side[7..11];

    if !cache.contains_key(m1) {
        step(m1, input, cache);
    }

    if !cache.contains_key(m2) {
        step(m2, input, cache);
    }

    if right_side.chars().nth(5).unwrap() == '+' {
        cache.insert(monkey.to_string(), cache[m1] + cache[m2]);
    } else if right_side.chars().nth(5).unwrap() == '-' {
        cache.insert(monkey.to_string(), cache[m1] - cache[m2]);
    } else if right_side.chars().nth(5).unwrap() == '*' {
        cache.insert(monkey.to_string(), cache[m1] * cache[m2]);
    } else if right_side.chars().nth(5).unwrap() == '/' {
        cache.insert(monkey.to_string(), cache[m1] / cache[m2]);
    }
}

fn step_str(monkey: &str, input: &HashMap<String, String>, cache: &mut HashMap<String, String>) {
    let right_side = &input[monkey];

    if monkey == "humn" {
        cache.insert("humn".to_string(), "x".to_string());
        return;
    }

    if let Ok(number) = right_side.parse::<i64>() {
        cache.insert(monkey.to_string(), right_side.clone());
        return;
    }

    let m1 = &right_side[0..4];
    let m2 = &right_side[7..11];

    if !cache.contains_key(m1) {
        step_str(m1, input, cache);
    }

    if !cache.contains_key(m2) {
        step_str(m2, input, cache);
    }

    let a = cache[m1].parse::<i64>();
    let b = cache[m2].parse::<i64>();

    if right_side.chars().nth(5).unwrap() == '+' {
        if a.is_ok() && b.is_ok() {
            cache.insert(monkey.to_string(), (a.unwrap() + b.unwrap()).to_string());
        } else {
            cache.insert(monkey.to_string(), format!("({} + {})", cache[m1], cache[m2]));
        }
    } else if right_side.chars().nth(5).unwrap() == '-' {
        if a.is_ok() && b.is_ok() {
            cache.insert(monkey.to_string(), (a.unwrap() - b.unwrap()).to_string());
        } else {
            cache.insert(monkey.to_string(), format!("({} - {})", cache[m1], cache[m2]));
        }
    } else if right_side.chars().nth(5).unwrap() == '*' {
        if a.is_ok() && b.is_ok() {
            cache.insert(monkey.to_string(), (a.unwrap() * b.unwrap()).to_string());
        } else {
            cache.insert(monkey.to_string(), format!("({} * {})", cache[m1], cache[m2]));
        }
    } else if right_side.chars().nth(5).unwrap() == '/' {
        if a.is_ok() && b.is_ok() {
            cache.insert(monkey.to_string(), (a.unwrap() / b.unwrap()).to_string());
        } else {
            cache.insert(monkey.to_string(), format!("({} / {})", cache[m1], cache[m2]));
        }
    }
}

fn split(input: &str) -> (&str, char, &str) {
    let input = &input[1..input.len() - 1];

    let mut counter = 0u8;
    let mut first = "";
    let mut i = 0;

    'outer: while i < input.len() {
        if input.chars().nth(i).unwrap() == '(' {
            counter += 1;
        } else if input.chars().nth(i).unwrap() == ')' {
            counter -= 1;
        } else if counter == 0 {
            if i == 0 {
                for j in 1..input.len() {
                    let res = input[0..j].parse::<i64>();
                    if res.is_ok() {
                        first = &input[0..j];
                    } else {
                        break 'outer;
                    }
                }
            } else {
                first = &input[0..i];
                break;
            }
        }

        i += 1;
    }

    let op = input.chars().nth(first.len() + 1).unwrap();
    let end = &input[first.len() + 3..input.len()];

    (first, op, end)
}

fn main() {
    println!("{}", 3243420789506i64 + 215);
    let input = stdin().lock().lines().map(|l| {
        let line = l.unwrap();
        let mut splitted = line.split(": ");

        (splitted.next().unwrap().to_string(), splitted.next().unwrap().to_string())
    }).collect::<HashMap<_, _>>();

    let mut cache = HashMap::<String, i64>::new();
    step("root", &input, &mut cache);

    println!("{}", cache["root"]);

    // let mut input = input.clone();
    let mut cache = HashMap::<String, String>::new();

    step_str("hppd", &input, &mut cache);
    step_str("czdp", &input, &mut cache);

    let mut left = cache["hppd"].clone();
    let mut right = cache["czdp"].parse::<i64>().unwrap();

    loop {
        if left == "humn" {
            break;
        }
        println!("{}", left);

        let (first, op, second) = split(left.as_str());

        println!("{} {} {} = {}", first, op, second, right);

        let first_num = first.parse::<i64>();

        if first_num.is_ok() {
            let first_num = first_num.unwrap();
            if op == '*' {
                right /= first_num;
            }

            if op == '/' {
                right = first_num / right;
            }

            if op == '+' {
                right -= first_num;
            }
            if op == '-' {
                right = first_num - right;
            }
            left = second.to_string();
            continue;
        }

        let second_num = second.parse::<i64>().unwrap();
        if op == '*' {
            right /= second_num;
        }

        if op == '/' {
            right *= second_num;
        }

        if op == '+' {
            right -= second_num;
        }
        if op == '-' {
            right += second_num;
        }
        left = first.to_string();
    }

    println!("{right}", );
}
