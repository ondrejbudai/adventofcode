use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_map() -> Vec<Vec<Vec<char>>> {
    let f = File::open("input").unwrap();
    let lines = BufReader::new(f).lines();

    let mut chunks = Vec::new();
    let mut current_chunk = Vec::new();

    for line in lines {
        let line = line.unwrap();
        if !line.is_empty() {
            current_chunk.push(line.chars().collect());
        } else {
            chunks.push(current_chunk);
            current_chunk = vec![];
        }
    }

    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }

    chunks
}

struct Smudger {
    orig: Vec<Vec<char>>,
    curr: usize,
}
impl Iterator for Smudger {
    type Item = Vec<Vec<char>>;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    // We use Self::Item in the return type, so we can change
    // the type without having to update the function signatures.
    fn next(&mut self) -> Option<Self::Item> {
        let size = self.orig.len() * self.orig[0].len();
        if self.curr == size {
            return None
        }

        let mut current = self.orig.clone();
        let cur_symbol = current.get_mut(self.curr / self.orig[0].len()).unwrap().get_mut(self.curr % self.orig[0].len()).unwrap();
        *cur_symbol = if *cur_symbol == '#' {
            '.'
        } else {
            '#'
        };

        self.curr += 1;

        Some(current)
    }
}

fn find_reflection(m: &Vec<Vec<char>>) -> usize {
    't:
    for i in 0..m.len()-1 {
        let num = (i + 1).min(m.len() - i - 1);

        for j in 0..num {
            if m[i-j] != m[i+j+1] {
                continue 't;
            }
        }

        return (i + 1) * 100;
    }

    't:
    for i in 0..m[0].len()-1 {
        let num = (i + 1).min(m[0].len() - i - 1);

        for j in 0..num {
            for y in 0..m.len() {
                if m[y][i-j] != m[y][i+j+1] {
                    continue 't;
                }
            }

        }

        return i + 1;
    }

    0
}


fn main() {
    let map = read_map();

    let sum: usize = map.iter().enumerate().map(|(i, m)| {
        let smudger = Smudger{
            curr: 0,
            orig: m.clone(),
        };

        let orig_reflection = find_reflection(m);

        'smudger:
        for m in smudger {
            let m = m;

            't:
            for i in 0..m.len()-1 {
                let num = (i + 1).min(m.len() - i - 1);

                for j in 0..num {
                    if m[i-j] != m[i+j+1] {
                        continue 't;
                    }
                }
                let reflection = (i + 1) * 100;

                if orig_reflection == reflection {
                    continue 't;
                } else {
                    return reflection;
                }
            }

            't:
            for i in 0..m[0].len()-1 {
                let num = (i + 1).min(m[0].len() - i - 1);

                for j in 0..num {
                    for y in 0..m.len() {
                        if m[y][i-j] != m[y][i+j+1] {
                            continue 't;
                        }
                    }

                }
                let reflection = i + 1;

                if orig_reflection == reflection {
                    continue 't;
                } else {
                    return reflection;
                }
            }
        }
        0
    }).sum();

    println!("{sum}");
}
