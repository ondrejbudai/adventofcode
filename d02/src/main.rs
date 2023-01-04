use std::collections::HashMap;

// A, X rock 1
// B, Y paper 2
// C, Z scissors 3
fn main() {
    let match_scores = HashMap::from([
        ("A X", 3),
        ("A Y", 1),
        ("A Z", 2),
        ("B X", 1),
        ("B Y", 2),
        ("B Z", 3),
        ("C X", 2),
        ("C Y", 3),
        ("C Z", 1),
    ]);

    let shape_scores = HashMap::from([
        ('X', 0),
        ('Y', 3),
        ('Z', 6),
    ]);

    use std::io::{self, BufRead};
    let stdin = io::stdin();
    let mut score = 0;
    for rawline in stdin.lock().lines() {
        let line = rawline.unwrap();
        let my_shape = line.chars().nth(2).unwrap();

        score += match_scores.get(line.as_str()).unwrap() + shape_scores.get(&my_shape).unwrap();
    }

    println!("{}", score)

}
