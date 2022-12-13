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

fn part1(lines: &[String]) -> Result<String> {
    let width = lines[0].len();
    let mut height_map = vec![0; lines.len() * width];
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, line) in lines.iter().enumerate() {
        for (j, h) in line.chars().enumerate() {
            match h {
                'S' => start = (j, i),
                'E' => end = (j, i),
                'a'..='z' => height_map[i * width + j] = (h as u32 - 'a' as u32),
                _ => unreachable!("Unsupported height")
            }
        }
    }

    todo!("Part 1")
}

fn part2(lines: &[String]) -> Result<String> {
    todo!("Part 2")
}
