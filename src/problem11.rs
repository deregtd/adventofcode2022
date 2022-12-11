use std::fs::File;
use std::io::prelude::*;

// Monkey 0:
//   Starting items: 89, 84, 88, 78, 70
//   Operation: new = old * 5
//   Test: divisible by 7
//     If true: throw to monkey 6
//     If false: throw to monkey 7


// #[derive(Copy, Clone)]
// #[derive(PartialEq)]
#[derive(Debug)]
struct Monkey {
    item_worries: Vec<i64>,
    op_parts: Vec<String>,
    test_div: i64,
    if_true_throw: i32,
    if_false_throw: i32,

    inspect_cnt: i64,
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem11.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let monkeys_raw = contents.split("\n\n").collect::<Vec<_>>();
    let mut monkeys = vec![];
    let mut comden = 1;
    for mr in monkeys_raw {
        let lines = mr.split("\n").collect::<Vec<_>>();
        let m = Monkey {
            item_worries: lines[1].split_once(": ").unwrap().1.split(", ").map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>(),
            op_parts: String::from(lines[2].split_once(" = ").unwrap().1).split(" ").map(|s| String::from(s)).collect::<Vec<_>>(),
            test_div: lines[3].split_once(" by ").unwrap().1.parse::<i64>().unwrap(),
            if_true_throw: lines[4].split_once(" monkey ").unwrap().1.parse::<i32>().unwrap(),
            if_false_throw: lines[5].split_once(" monkey ").unwrap().1.parse::<i32>().unwrap(),

            inspect_cnt: 0,
        };
        comden *= m.test_div;
        monkeys.push(m);
    }

    // Part 1
    // for _r in 0..20 {
    // Part 2
    for _r in 0..10000 {
        for mi in 0..monkeys.len() {
            let mut m = &mut monkeys[mi];
            let mut out_worries = vec![];
            for mut worry in m.item_worries.clone() {
                m.inspect_cnt = m.inspect_cnt + 1;

                // all ops start with "old"
                let operand = if m.op_parts[2] == "old" { worry } else { m.op_parts[2].parse::<i64>().unwrap() };
                if m.op_parts[1] == "*" {
                    worry = worry * operand;
                } else if m.op_parts[1] == "/" {
                    worry = worry / operand;
                } else if m.op_parts[1] == "+" {
                    worry = worry + operand;
                } else if m.op_parts[1] == "-" {
                    worry = worry - operand;
                }

                // Pt 1
                // worry = worry / 3;
                // Pt 2
                worry = worry % comden;

                let is_div = (worry % m.test_div) == 0;
                out_worries.push([if is_div { m.if_true_throw } else { m.if_false_throw } as i64, worry]);
            }
            m.item_worries = vec![];
            for ow in out_worries {
                monkeys[ow[0] as usize].item_worries.push(ow[1]);
            }
        }
    }

    let mut inspects = monkeys.iter().map(|m| m.inspect_cnt).collect::<Vec<_>>();
    inspects.sort();
    println!("{:?}", inspects[inspects.len() - 2] * inspects[inspects.len() - 1]);

    // for m in monkeys {
    //     println!("{:?}", m);
    // }

    Ok(())
}
