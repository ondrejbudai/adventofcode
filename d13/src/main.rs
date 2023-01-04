use std::cmp::Ordering;
use std::io::{BufRead, stdin};
use std::str::FromStr;

struct BracketIterator<'a>(&'a str);

impl<'a> Iterator for BracketIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let mut brackets = 0;
        for i in 0..self.0.chars().count() {
            let c = self.0.chars().nth(i).unwrap();
            if c == '[' {
                brackets += 1;
            } else if c == ']' {
                brackets -= 1;
            } else if c == ',' && brackets == 0 {
                let ret = self.0.get(0..i).unwrap();
                self.0 = self.0.get(i + 1..).unwrap();
                return Some(ret);
            }
        }

        if self.0.chars().count() > 0 {
            let ret = self.0;
            self.0 = &"";
            return Some(ret);
        }

        None
    }
}

enum ListOrValue {
    List(Vec<ListOrValue>),
    Value(u16),
}

impl PartialEq for ListOrValue {
    fn eq(&self, other: &Self) -> bool {
        if let ListOrValue::List(a) = self {
            if let ListOrValue::List(b) = other {
                return a == b;
            }
        }

        if let ListOrValue::Value(a) = self {
            if let ListOrValue::Value(b) = other {
                return *a == *b;
            }
        }

        false
    }
}

fn compare_vecs(a: &Vec<ListOrValue>, b: &Vec<ListOrValue>) -> Option<bool> {
    for (aval, bval) in a.iter().zip(b.iter()) {
        let maybe_ordered = aval.in_correct_order(bval);

        if let Some(_) = maybe_ordered {
            return maybe_ordered;
        }
    }

    if a.len() < b.len() {
        return Some(true);
    } else if a.len() > b.len() {
        return Some(false);
    }

    None
}

impl ListOrValue {
    fn in_correct_order(&self, other: &Self) -> Option<bool> {
        match self {
            ListOrValue::List(a) => {
                match other {
                    ListOrValue::List(b) => {
                        return compare_vecs(a, b);
                    }
                    ListOrValue::Value(b) => {
                        let b_vec = vec![ListOrValue::Value(*b)];
                        return compare_vecs(a, &b_vec);
                    }
                }
            }
            ListOrValue::Value(a) => {
                match other {
                    ListOrValue::List(b) => {
                        let a_vec = vec![ListOrValue::Value(*a)];
                        return compare_vecs(&a_vec, b);
                    }
                    ListOrValue::Value(b) => {
                        if *a < *b {
                            return Some(true);
                        } else if *a > *b {
                            return Some(false);
                        }
                    }
                }
            }
        }


        if let ListOrValue::Value(a) = self {
            if let ListOrValue::Value(b) = other {
                if *a < *b {
                    return Some(true);
                }
            }
        }

        None
    }
}

impl FromStr for ListOrValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().nth(0).unwrap() == '[' {
            let items = BracketIterator(s.get(1..s.len() - 1).unwrap());

            let vector = items.map(|it| it.parse::<ListOrValue>().unwrap()).collect::<Vec<ListOrValue>>();

            return Ok(ListOrValue::List(vector));
        }

        Ok(ListOrValue::Value(s.parse::<u16>().unwrap()))
    }
}

fn main() {

    let mut lines = stdin().lock().lines();

    // let mut index = 1;
    // let mut sum = 0;
    // loop {
    //     let l1 = lines.next().unwrap().unwrap().parse::<ListOrValue>().unwrap();
    //     let l2 = lines.next().unwrap().unwrap().parse::<ListOrValue>().unwrap();
    //
    //     let res = l1.in_correct_order(&l2);
    //
    //     match res {
    //         None => {
    //             println!("wtf");
    //         }
    //         Some(res) => {
    //             if res {
    //                 sum += index;
    //             }
    //         }
    //     }
    //
    //
    //     if let None = lines.next() {
    //         break;
    //     }
    //
    //     index += 1;
    // }
    // println!("{}", sum);

    let mut packets = vec![
      ListOrValue::List(vec![ListOrValue::List(vec![ListOrValue::Value(2)])]),
      ListOrValue::List(vec![ListOrValue::List(vec![ListOrValue::Value(6)])]),
    ];
    loop {
        packets.push(lines.next().unwrap().unwrap().parse::<ListOrValue>().unwrap());
        packets.push(lines.next().unwrap().unwrap().parse::<ListOrValue>().unwrap());


        if let None = lines.next() {
            break;
        }
    }

    packets.sort_by(|a, b| {
        match a.in_correct_order(b) {
            None => {
                panic!();
            }
            Some(true) => {
                Ordering::Less
            }
            Some(false) => {
                Ordering::Greater
            }
        }
    });

    let mut product = 1usize;

    packets.iter().enumerate().for_each(|(index, value)| {
       if *value == ListOrValue::List(vec![ListOrValue::List(vec![ListOrValue::Value(2)])]) || *value == ListOrValue::List(vec![ListOrValue::List(vec![ListOrValue::Value(6)])]) {
           product *= index + 1;
       }
    });

    println!("{}", product);
}
