use std::{fs::File, io::{Result, BufReader, BufRead}, ops::RangeInclusive};

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

fn parse_range(rg: &str) -> RangeInclusive<usize> {
    let (start, end) = rg.split_once("-").unwrap();
    RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap())
}

fn part1(lines: &[String]) -> Result<String> {
    let mut count = 0;
    for line in lines {
        let (elfe_a, elfe_b) = line.split_once(",")
            .map(|(a, b)| (parse_range(&a), parse_range(&b))).unwrap();
        if (elfe_a.contains(elfe_b.start()) && elfe_a.contains(elfe_b.end()))
        || (elfe_b.contains(elfe_a.start()) && elfe_b.contains(elfe_a.end())) {
            count += 1;
        }
    }
    Ok(format!("Full Overlaps = {}", count))
}

fn part2(lines: &[String]) -> Result<String> {
    let mut count = 0;
    for line in lines {
        let (elfe_a, elfe_b) = line.split_once(",")
            .map(|(a, b)| (parse_range(&a), parse_range(&b))).unwrap();
        if elfe_a.contains(elfe_b.start()) || elfe_a.contains(elfe_b.end())
        || elfe_b.contains(elfe_a.start()) || elfe_b.contains(elfe_a.end()) {
            count += 1;
        }
    }
    Ok(format!("Overlaps = {}", count))
}
