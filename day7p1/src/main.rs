use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    cards: String,
    typ: Type,
    bet: u32,
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.typ != other.typ {
            return self.typ.cmp(&other.typ);
        }

        for (a, b) in self.cards.chars().zip(other.cards.chars()) {
            if a != b {
                return a.cmp(&b);
            }
        }

        return Ordering::Equal
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

fn get_type(cards: &str) -> Type {
    let mut map: HashMap<char, u8> = HashMap::new();
    for card in cards.chars() {
        let entry = map.entry(card).or_insert(0);
        *entry += 1;
    }

    if map.len() == 1 {
        return Type::FiveOfAKind;
    }
    if map.len() == 2 {
        let (_, count) = map.iter().next().unwrap();
        if *count == 4 || *count == 1 {
            return Type::FourOfAKind;
        }

        return Type::FullHouse;
    }
    if map.len() == 3 {
        if map.iter().filter(|(_, n)| **n  == 3).count() > 0 {
            return Type::ThreeOfAKind;
        }
        return Type::TwoPair;
    }

    if map.iter().filter(|(_, n)| **n  == 2).count() > 0 {
        return Type::OnePair;
    }

    Type::HighCard
}

fn parse_hand(line: &str) -> Hand {
    let (cards, bet) = line.split_once(" ").unwrap();

    let cards: String = cards.chars().map(|x| {
        match x {
            '2' => 'A',
            '3' => 'B',
            '4' => 'C',
            '5' => 'D',
            '6' => 'E',
            '7' => 'F',
            '8' => 'G',
            '9' => 'H',
            'T' => 'I',
            'J' => 'J',
            'Q' => 'K',
            'K' => 'L',
            'A' => 'M',
            _ => panic!("unknown card")
        }
    }).collect();

    Hand{
        typ: get_type(cards.as_str()),
        cards: cards,
        bet: bet.parse().unwrap(),
    }
}

fn main() {
    let f = File::open("input").unwrap();
    let f = BufReader::new(f);
    let mut map = f.lines().map(|line| {
        let line = line.unwrap();

        parse_hand(line.as_str())
    }).collect::<Vec<Hand>>();

    map.sort();

    let sum: u32 = map.iter().enumerate().map(|(order, hand)| {
        (order + 1) as u32 * hand.bet
    }).sum();

    println!("{sum}");
}
