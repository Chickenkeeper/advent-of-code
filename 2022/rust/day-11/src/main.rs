use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
enum Operator {
    Old,
    New(usize),
}

#[derive(Clone, Debug)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    operator: Operator,
    test_divisor: usize,
    if_true: usize,
    if_false: usize,
}

fn load_monkeys(rounds: usize, worry_divisor: usize) -> Result<usize, Box<dyn std::error::Error>> {
    let file = File::open("input.txt").map_err(|e| format!("Error opening input.txt: {e}"))?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut monkeys = Vec::new();
    let mut worry_limiter = 1;

    loop {
        if let Some(_) = lines.next() {
            let items = lines
                .next()
                .unwrap()?
                .trim()
                .trim_start_matches("Starting items:")
                .split(',')
                .map(|n| n.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let o = lines.next().unwrap()?;
            let mut op = o
                .trim()
                .trim_start_matches("Operation: new = old")
                .split_whitespace();
            let operation = match op.next().unwrap().trim() {
                "*" => Operation::Multiply,
                _ => Operation::Add,
            };
            let oper = op.next().unwrap().trim();
            let operator = match oper {
                "old" => Operator::Old,
                _ => Operator::New(oper.parse::<usize>()?),
            };
            let test_divisor = lines
                .next()
                .unwrap()?
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()?;
            worry_limiter *= test_divisor;
            let if_true = lines
                .next()
                .unwrap()?
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()?;
            let if_false = lines
                .next()
                .unwrap()?
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<usize>()?;

            monkeys.push(Monkey {
                items,
                operation,
                operator,
                test_divisor,
                if_true,
                if_false,
            });

            lines.next();
        } else {
            break;
        }
    }

    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            let mut monkey = monkeys[m].clone();

            for i in monkey.items.iter() {
                let mut item = *i;

                match monkey.operation {
                    Operation::Add => match monkey.operator {
                        Operator::Old => item += item,
                        Operator::New(n) => item += n,
                    },
                    Operation::Multiply => match monkey.operator {
                        Operator::Old => item = (item % worry_limiter) * (item % worry_limiter),
                        Operator::New(n) => item = item * n,
                    },
                }

                item /= worry_divisor;

                if item % monkey.test_divisor == 0 {
                    monkeys[monkey.if_true].items.push(item);
                } else {
                    monkeys[monkey.if_false].items.push(item);
                }

                inspections[m] += 1;
            }

            monkey.items.clear();
            monkeys[m] = monkey;
        }
    }

    inspections.sort();
    let mb: Vec<_> = inspections.iter().rev().take(2).collect();

    return Ok(*mb[0] * *mb[1]);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let part_1 = load_monkeys(20, 3)?;
    let part_2 = load_monkeys(10000, 1)?;

    println!("Part 1 - Monkey business after 20 rounds: {part_1}");
    println!("Part 2 - Monkey business after 10,000 rounds: {part_2}");
    return Ok(());
}
