use std::fs;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{eof, map_res, opt};
use nom::IResult;
use nom::multi::separated_list0;
use nom::number::complete::u32;

struct Peak {
    red: u32,
    green: u32,
    blue: u32,
}
struct Game {
    id: u32,
    peaks: Vec<Peak>
}

fn parse_color(s: &str) -> IResult<&str, (u32, &str)> {
    let (s, num) = map_res(digit1, str::parse)(s)?;
    let (s, _) = tag(" ")(s)?;
    let (s, color) = alt((tag("red"), tag("green"), tag("blue")))(s)?;

    Ok((s, (num, color)))
}

fn parse_peak(s: &str) -> IResult<&str, Peak> {
    let (s, colors) = separated_list0(tag(", "), parse_color)(s)?;

    let mut peak = Peak{
        red: 0,
        green: 0,
        blue: 0,
    };

    for color in colors {
        match color.1 {
            "green" => peak.green = color.0,
            "red" => peak.red = color.0,
            "blue" => peak.blue = color.0,
            _ => panic!("unknown color!")
        }
    }

    Ok((s, peak))
}
fn parse_line(line: &str) -> IResult<&str, Game> {
    let (line, _) = tag("Game ")(line)?;
    let (line, id) = map_res(digit1, str::parse)(line)?;
    let (line, _) = tag(": ")(line)?;

    let (line, peaks) = separated_list0(tag("; "), parse_peak)(line)?;

    eof(line)?;

    Ok((line,Game{id, peaks }))
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let games = input.lines().map(|x| parse_line(x).unwrap().1).collect::<Vec<_>>();
    let mut sum = 0;
    'outer:
    for game in games {
        let mut rmin = 0;
        let mut gmin = 0;
        let mut bmin = 0;
        for peak in game.peaks {
            rmin = rmin.max(peak.red);
            gmin = gmin.max(peak.green);
            bmin = bmin.max(peak.blue);
        }
        sum += rmin * gmin * bmin   ;
    }

    println!("{sum}");
}
