use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
enum MathOp {
    Add,
    Sub,
    Mul,
    Div,
    Equals,
    None,
}

#[derive(Clone)]
struct Monkey {
    name: String,
    value: i64,
    math_op: MathOp,
    dependencies: Vec<String>,
}

impl Monkey {
    fn parse(v: &str) -> Self {
        // wbrf: 17
        // ldhv: vpdn * lpth
        let parts = v.split_once(": ").unwrap();
        let id = parts.0;
        let opparts = parts.1.split(" ").collect::<Vec<_>>();
        if opparts.len() == 1 {
            return Monkey {
                name: String::from(id),
                value: opparts[0].parse::<i64>().unwrap(),
                math_op: MathOp::None,
                dependencies: vec![],
            };
        } else {
            return Monkey {
                name: String::from(id),
                value: -1,
                math_op: match opparts[1] { "+" => MathOp::Add, "-" => MathOp::Sub, "*" => MathOp::Mul, "/" => MathOp::Div, &_ => todo!() },
                dependencies: vec![String::from(opparts[0]), String::from(opparts[2])],
            };
        }
    }

    fn value(&self, monkey_lookup: &HashMap<String, Monkey>) -> String {
        if self.dependencies.len() == 0 {
            return self.value.to_string();
        }

        let vals = self.dependencies.iter().map(|dn| monkey_lookup.get(dn).unwrap().value(monkey_lookup)).collect::<Vec<_>>();
        return String::from("(") + &vals[0].clone() + &String::from(match self.math_op {
            MathOp::Add => "+",
            MathOp::Sub => "-",
            MathOp::Mul => "*",
            MathOp::Div => "/",
            MathOp::Equals => "==",
            MathOp::None => todo!(),
        }) + &vals[1] + ")";
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/problem21.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let monkeys_raw = contents.split("\n").map(|r| Monkey::parse(r));

    let mut monkey_lookup = HashMap::new();
    for m in monkeys_raw {
        monkey_lookup.insert(m.name.clone(), m);
    }
    monkey_lookup.get_mut("root").unwrap().math_op = MathOp::Equals;
    monkey_lookup.get_mut("humn").unwrap().value = 9999;

    println!("{}", monkey_lookup.get("root").unwrap().value(&monkey_lookup));

    // Then take the equation, replace 9999 with "x" and "==" with "=" and drop it into https://www.mathpapa.com/simplify-calculator/, and you get the answer.

    Ok(())
}
