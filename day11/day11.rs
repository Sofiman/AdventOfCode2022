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
    /// Items queued
    items: Vec<usize>,
    /// Inspect operation on the current item's worry level
    op: Fn(usize) -> usize,
    /// Next monkey to throw to according to the worry level of the current item
    next: Fn(usize) -> usize
}

fn undertsand_monkey(desc: &[&str]) -> Monkey {
    todo!()
}

fn part1(lines: &[String]) -> Result<String> {
    todo!("Part 1")
}

fn part2(lines: &[String]) -> Result<String> {
    todo!("Part 2")
}
