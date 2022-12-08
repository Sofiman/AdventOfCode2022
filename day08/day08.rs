use std::{fs::File, io::{Result, BufReader, BufRead}};

fn main() -> Result<()> {
    let f = File::open("e.txt")?;
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

fn part1(lines: &[String]) -> Result<String> {
    let width = 99;
    let mut grid = vec![0; width * lines.len()];
    for (i, line) in lines.iter().enumerate() {
        let mut j = 0;
        for c in line.chars() {
            grid[i * width + j] = c.to_digit(10).unwrap();
            j += 1;
        }
    }
    let mut visible = 0;

    for (i, line) in lines.iter().enumerate() {
        let mut j = 1;
        while j < line.len() && grid[i * width + j - 1] < grid[i * width + j] {
            j += 1;
        }
        visible += j;
    }

    Ok(format!("{}", visible))
}

fn part2(lines: &[String]) -> Result<String> {
    todo!("Part 2")
}
