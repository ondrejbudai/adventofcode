use std::io::{Read, stdin};
use std::collections::LinkedList;

// const MARKER_LENGTH: usize = 4;
const MARKER_LENGTH: usize = 14;

fn main() {
    let mut input = stdin().lock().bytes();
    let mut lookback: LinkedList<u16> = LinkedList::new();

    for (i, current_byte) in input.enumerate() {
        let current = current_byte.unwrap();
        let mut cut = 0;

        for (j, cur) in lookback.iter().enumerate() {
            if current == *cur {
                cut = j + 1;
                break;
            }
        }

        for _ in 0..cut {
            lookback.pop_front();
        }
        lookback.push_back(current);

        if lookback.len() == MARKER_LENGTH {
            println!("{}", i + 1);
            break;
        }
    }
}
