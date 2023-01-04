use std::io::{BufRead, stdin};

fn main() {
    let nums = stdin().lock().lines().map(|l| l.unwrap().parse::<i64>().unwrap()).map(|a| a * 811589153).collect::<Vec<_>>();
    let mut data = nums.iter().enumerate().collect::<Vec<_>>();
    let len = nums.len();
    println!("{:?}", data.iter().map(|a| { a.1 }).collect::<Vec<_>>());
    for _ in 0..10 {
        for (i, amount) in nums.iter().enumerate() {
            // if i % 100 == 0 {
            //     println!("{i}");
            // }
            // if i == 10 { break; }
            if *amount == 0 {
                continue;
            }
            let pos = data.iter().position(|cur| cur.0 == i).unwrap();


            // let mut new_pos = ((pos as i64 + len as i64 + amount) % len as i64) as usize;
            let mut new_pos = pos as i64 + amount;
            // while new_pos < 0 {
            //     new_pos += len as i64 - 1;
            // }


            // while new_pos >= len as i64 {
            //     new_pos -= len as i64 - 1;
            // }

            while new_pos < 0 {
                new_pos -= (new_pos / len as i64 - 1) * (len as i64 - 1);
            }

            while new_pos >= len as i64 {
                new_pos -= (new_pos / len as i64) * (len as i64 - 1);
            }

            let mut new_pos = new_pos as usize;

            // let mut new_pos = (new_pos % len as i64) as usize;
            // println!("Moving index {} (value: {}, current index: {}) by {}", i, data[pos].1, pos, amount);

            if *amount > 0 {
                if new_pos > pos {
                    let backup = data[pos];
                    for ix in pos + 1..=new_pos {
                        data[ix - 1] = data[ix];
                    }
                    data[new_pos] = backup;
                } else {
                    // new_pos += 1;
                    let backup = data[pos];
                    for ix in (new_pos..pos).rev() {
                        data[ix + 1] = data[ix];
                    }
                    data[new_pos] = backup;
                }
            } else {
                // if new_pos == 0 {
                //     new_pos = len;
                // }
                if new_pos < pos {
                    let backup = data[pos];
                    for ix in (new_pos..pos).rev() {
                        data[ix + 1] = data[ix];
                    }
                    data[new_pos] = backup;
                } else {
                    // new_pos -= 1;
                    let backup = data[pos];
                    for ix in pos + 1..=new_pos {
                        data[ix - 1] = data[ix];
                    }
                    data[new_pos] = backup;
                }
            }
            // println!("{:?}", data.iter().map(|a| { a.1 }).collect::<Vec<_>>());
        }
    }

    let pos = data.iter().position(|cur| *cur.1 == 0).unwrap();

    // println!("{}", pos);
    println!("{}", data[(pos + 1000) % len].1);
    println!("{}", data[(pos + 2000) % len].1);
    println!("{}", data[(pos + 3000) % len].1);


    println!("{}", data[(pos + 1000) % len].1 + data[(pos + 2000) % len].1+ data[(pos + 3000) % len].1)
}
