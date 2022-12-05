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

fn parse_initial_params(lines: &[String], idx: &mut usize) -> Vec<Vec<char>> {
    let mut i = *idx;
    while !lines[i].is_empty() { // Collect all the stacks
        i += 1;
    }
    *idx = i + 1;
    i -= 1; // remove the empty line
    let labels = &lines[i];
    let nb_stacks: usize = labels[(labels.len() - 3)..].trim().parse().unwrap();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; nb_stacks];

    while i > 0 {
        i -= 1;
        let mut current_stack = 0;
        for c in lines[i].chars().skip(1).step_by(4) {
            if c != ' ' {
                stacks[current_stack].push(c);
            }
            current_stack += 1;
        }
    }
    stacks
}

/// (count, from, to)
fn parse_move(s: &str) -> (usize, usize, usize) {
    let s = &s[5..]; // skip the move keyword
    let (count, s) = s.split_once(" ").unwrap();
    let count: usize = count.parse().unwrap();
    let s = &s[5..];
    let (from, s) = s.split_once(" ").unwrap();
    let from: usize = from.parse().unwrap();
    let s = &s[3..];
    let to: usize = s.parse().unwrap();
    (count, from, to)
}

fn part1(lines: &[String]) -> Result<String> {
    let mut idx = 0;
    let mut stacks = parse_initial_params(lines, &mut idx);

    for l in &lines[idx..] {
        let (count, from, to) = parse_move(l);
        for _ in 0..count {
            let item = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(item);
        }
    }

    let tops: String = stacks.iter_mut().map(|v| v.pop()).flatten().collect();
    Ok(format!("{}", tops))
}

fn part2(lines: &[String]) -> Result<String> {
    let mut idx = 0;
    let mut stacks = parse_initial_params(lines, &mut idx);

    for l in &lines[idx..] {
        let (count, from, to) = parse_move(l);
        let source = &mut stacks[from - 1];
        let mut crates = source.split_off(source.len() - count);
        stacks[to - 1].append(&mut crates);
    }

    let tops: String = stacks.iter_mut().map(|v| v.pop()).flatten().collect();
    Ok(format!("{}", tops))
}
