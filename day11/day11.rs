use std::{fs::File, io::{Result, BufReader, BufRead}};

fn main() -> Result<()> {
    let f = File::open("input.txt")?;
    let lines: Vec<String> = BufReader::new(f).lines().flatten().collect();

    println!("Result for Part1:");
    let p1 = part1(&lines)?;
    println!("{}", p1);

    println!();
    println!("Result for Part2:");
    let p2 = part2(&lines)?;
    println!("{}", p2);

    Ok(())
}

/// Basically a virutal machine running some algorithm
struct Monkey {
    id: usize,
    /// Items queued
    items: Vec<usize>,
    /// Inspect operation on the current item's worry level
    op: Box<dyn Fn(usize) -> usize>,
    /// Next monkey to throw to according to the worry level of the current item
    next: Box<dyn Fn(usize) -> usize>,
    divisible_target: usize,
    /// Total number of inspected items
    inspected: usize
}

fn understand_monkey(desc: &[&str]) -> Monkey {
    let id: usize = desc[0][7..].strip_suffix(":").unwrap().parse().unwrap();
    let items: Vec<usize> = desc[1][18..].split(", ")
        .map(|item| item.parse().unwrap()).collect();
    let (operator, operand) = desc[2][23..].split_once(" ").unwrap();
    let divisible_target: usize = desc[3][21..].parse().unwrap();
    let true_to: usize = desc[4][29..].parse().unwrap();
    let false_to: usize = desc[5][30..].parse().unwrap();

    let op: Box<dyn Fn(usize) -> usize> = match operand {
        "old" => match operator {
            "*" => Box::new(|item| { item * item }),
            "+" => Box::new(|item| { item + item }),
            _ => unreachable!("Unsupported operation")
        },
        _ => {
            let operand: usize = operand.parse().unwrap();
            match operator {
                "*" => Box::new(move |item| { item * operand }),
                "+" => Box::new(move |item| { item + operand }),
                _ => unreachable!("Unsupported operation")
            }
        }
    };
    let next = Box::new(move |item| {
        if item % divisible_target == 0 {
            true_to
        } else {
            false_to
        }
    });

    Monkey { id, items, op, next, divisible_target, inspected: 0 }
}

fn part1(lines: &[String]) -> Result<String> {
    let mut monkeys = vec![];
    let mut state = vec![];
    for line in lines.iter() {
        if line.trim().is_empty() {
            let monkey = understand_monkey(&state);
            state.clear();
            monkeys.push(monkey);
        } else {
            state.push(line);
        }
    }
    monkeys.push(understand_monkey(&state));

    for round in 0..20 {
        for i in 0..monkeys.len() {
            let mut monkey = &mut monkeys[i];
            let mut to_add = vec![];
            for item in monkey.items.drain(..) {
                let item = (monkey.op)(item) / 3;
                let next = (monkey.next)(item);
                monkey.inspected += 1;
                to_add.push((next, item));
            }
            for (monkey, item) in to_add {
                monkeys[monkey].items.push(item);
            }
        }
    }

    let mut max = 0;
    let mut penultimate = 0;
    for monkey in monkeys.iter() {
        println!("Monkey {} inspected items {} times.", monkey.id, monkey.inspected);
        if monkey.inspected > max {
            penultimate = max;
            max = monkey.inspected;
        }
    }

    Ok(format!("total: {}", max * penultimate))
}

fn part2(lines: &[String]) -> Result<String> {
    let mut monkeys = vec![];
    let mut state = vec![];
    let mut m = 1;
    for line in lines.iter() {
        if line.trim().is_empty() {
            let monkey = understand_monkey(&state);
            state.clear();
            m *= monkey.divisible_target;
            monkeys.push(monkey);
        } else {
            state.push(line);
        }
    }
    let monkey = understand_monkey(&state);
    m *= monkey.divisible_target;
    monkeys.push(monkey);

    for round in 0..10000 {
        for i in 0..monkeys.len() {
            let mut monkey = &mut monkeys[i];
            let mut to_add = vec![];
            for item in monkey.items.drain(..) {
                let item = (monkey.op)(item) % m;
                let next = (monkey.next)(item);
                monkey.inspected += 1;
                to_add.push((next, item));
            }
            for (monkey, item) in to_add {
                monkeys[monkey].items.push(item);
            }
        }
    }

    let mut max = 0;
    let mut penultimate = 0;
    for monkey in monkeys.iter() {
        println!("Monkey {} inspected items {} times.", monkey.id, monkey.inspected);
        if monkey.inspected > max {
            penultimate = max;
            max = monkey.inspected;
        }
    }

    Ok(format!("total: {}", max * penultimate))
}
