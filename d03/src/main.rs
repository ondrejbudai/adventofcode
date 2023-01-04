fn main() {
    use std::io::{self, BufRead};
    let stdin = io::stdin();

    // let mut sum = 0;
    // for rawline in stdin.lock().lines() {
    //     let line = rawline.unwrap();
    //     let a = &line[..line.len()/2];
    //     let b = &line[line.len()/2..];
    //
    //     'lop: for a1 in a.chars() {
    //         for b1 in b.chars() {
    //             if a1 != b1 {
    //                 continue;
    //             }
    //             if a1 <= 'Z' {
    //                 sum += a1 as u32 - 38
    //             } else {
    //                 sum += a1 as u32 - 96
    //             }
    //
    //             break 'lop;
    //         }
    //     }
    // }
    //
    // println!("{}", sum)

    let mut sum = 0;
    let mut lines = stdin.lock().lines();

    loop {
        let line_raw = lines.next();
        match line_raw {
            None => break,
            Some(l) => {
                let l1 = l.unwrap();
                let l2 = lines.next().unwrap().unwrap();
                let l3 = lines.next().unwrap().unwrap();
                'lop: for c1 in l1.chars() {
                    for c2 in l2.chars() {
                        for c3 in l3.chars() {
                            if c1 != c2 || c2 != c3 {
                                continue;
                            }
                            if c1 <= 'Z' {
                                sum += c1 as u32 - 38
                            } else {
                                sum += c1 as u32 - 96
                            }

                            break 'lop;
                        }
                    }
                }
            }
        }
    }
    println!("{}", sum)
}
