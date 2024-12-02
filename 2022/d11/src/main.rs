use std::collections::VecDeque;

struct Monkey {
    items: VecDeque<u64>,
    operation: fn(u64) -> u64,
    divisible: u64,
    if_true: usize,
    if_false: usize,
    inspections: usize,
}

fn main() {
    // let mut monkeys: [Monkey; 4] = [
    //     Monkey {
    //         items: VecDeque::from([79, 98]),
    //         operation: |old| old * 19,
    //         divisible: 23,
    //         if_true: 2,
    //         if_false: 3,
    //         inspections: 0,
    //     },
    //     Monkey {
    //         items: VecDeque::from([54, 65, 75, 74]),
    //         operation: |old| old + 6,
    //         divisible: 19,
    //         if_true: 2,
    //         if_false: 0,
    //         inspections: 0,
    //     },
    //     Monkey {
    //         items: VecDeque::from([79, 60, 67]),
    //         operation: |old| old * old,
    //         divisible: 13,
    //         if_true: 1,
    //         if_false: 3,
    //         inspections: 0,
    //     },
    //     Monkey {
    //         items: VecDeque::from([74]),
    //         operation: |old| old + 3,
    //         divisible: 17,
    //         if_true: 0,
    //         if_false: 1,
    //         inspections: 0,
    //     }
    // ];

    let mut monkeys: [Monkey; 8] = [
        Monkey {
            items: VecDeque::from([84, 72, 58, 51]),
            operation: |old| old * 3,
            divisible: 13,
            if_true: 1,
            if_false: 7,
            inspections: 0,
        },
        Monkey {
            items: VecDeque::from([88, 58, 58]),
            operation: |old| old + 8,
            divisible: 2,
            if_true: 7,
            if_false: 5,
            inspections: 0,
        },
        Monkey {
            items: VecDeque::from([93, 82, 71, 77, 83, 53, 71, 89]),
            operation: |old| old * old,
            divisible: 7,
            if_true: 3,
            if_false: 4,
            inspections: 0,
        },
        Monkey {
            items: VecDeque::from([81, 68, 65, 81, 73, 77, 96]),
            operation: |old| old + 2,
            divisible: 17,
            if_true: 4,
            if_false: 6,
            inspections: 0,
        },
        Monkey {
            items: VecDeque::from([75, 80, 50, 73, 88]),
            operation: |old| old + 3,
            divisible: 5,
            if_true: 6,
            if_false: 0,
            inspections: 0,
        },
        Monkey {
            items: VecDeque::from([59, 72, 99, 87, 91, 81]),
            operation: |old| old * 17,
            divisible: 11,
            if_true: 2,
            if_false: 3,
            inspections: 0,
        },
        Monkey {
            items: VecDeque::from([86, 69]),
            operation: |old| old + 6,
            divisible: 3,
            if_true: 1,
            if_false: 0,
            inspections: 0,
        },
        Monkey {
            items: VecDeque::from([91]),
            operation: |old| old + 1,
            divisible: 19,
            if_true: 2,
            if_false: 5,
            inspections: 0,
        }
    ];

    let limiter = 19u64 * 3 * 11 * 5 * 17 * 7 * 2 * 13;


    for _ in 0..10000 {
        for m_id in 0..monkeys.len() {
            let items = monkeys[m_id].items.clone();
            monkeys[m_id].items.clear();

            monkeys[m_id].inspections += items.len();

            items.iter().for_each(|old| {
                let new = ((monkeys[m_id].operation)(*old) as f64) as u64 % limiter;
                if new % monkeys[m_id].divisible == 0 {
                    monkeys[monkeys[m_id].if_true].items.push_back(new);
                } else {
                    monkeys[monkeys[m_id].if_false].items.push_back(new);
                }
            });
        }
    }

    monkeys.iter().enumerate().for_each(| (id, monkey)| {
       println!("Monkey {}: {} inspections", id, monkey.inspections);
    });
}
