use std::io::{BufRead, stdin};

fn convert_from_snafu(input: &str) -> i64 {
    let mut result= 0;

    let mut pow = 5i64.pow(input.len() as u32 - 1);

    input.chars().for_each(|x| {
        let num = match x {
            '1' => 1i64,
            '2' => 2,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => unreachable!()
        } ;

        result += num * pow;
        pow /= 5;
    });

    result
}

fn convert_to_snafu(num: i64) -> String {
    let mut num = num;
    let mut result = String::new();
    while num > 0 {
        let a = num % 5;
        match a {
            0 => {
                result = format!("0{}", result);
            }
            1 => {
                result = format!("1{}", result);
            }
            2 => {
                result = format!("2{}", result);
            }
            3 => {
                result = format!("={}", result);
                num += 2;
            }
            4 => {
                result = format!("-{}", result);
                num += 1;
            }
            _ => unreachable!()
        }

        //2==0=0===02--210---1

        num /= 5;
    }
    result
}

fn main() {
    let sum: i64 = stdin().lock().lines().map(|l| {
        convert_from_snafu(l.unwrap().as_str())
    }).sum();

    println!("{}", convert_to_snafu(sum));


}
