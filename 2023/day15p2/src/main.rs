use std::fs;

fn hash(s: &str) -> u32 {
    let mut cur = 0u32;

    s.bytes().for_each(|x| {
        cur += x as u32;
        cur *= 17;
        cur %= 256;
    });

    cur
}

struct Lens {
    label: String,
    length: u32,
}

struct Box {
    lens: Vec<Lens>,
}

fn main() {
    let s = fs::read_to_string("input").unwrap();
    let mut boxes = vec![];
    for _ in 0..256 {
        boxes.push(Box { lens: Vec::new() });
    }
    s.trim().split(",").for_each(|cmd| {
        let (label, _) = cmd.split_once(['=', '-']).unwrap();
        let h = hash(label);
        let lens = &mut boxes.get_mut(h as usize).unwrap().lens;
        let i = lens.iter().position(|l| l.label == label);

        if cmd.chars().nth(label.len()).unwrap() == '=' {
            let length = cmd[label.len() + 1..].parse().unwrap();
            match i {
                None => lens.push(Lens { label: label.to_string(), length }),
                Some(i) => lens[i].length = length,
            }
        } else {
            match i {
                None => {}
                Some(i) => { lens.remove(i); }
            }
        }
    });

    let sum: u32 = boxes.iter().enumerate().map(|(i, b)| {
        (i as u32 + 1) * b.lens.iter().enumerate().map(|(i, l)| {
            (i as u32 + 1) * l.length
        }).sum::<u32>()
    }).sum();

    println!("{sum}");
}
