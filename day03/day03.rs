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

fn count_pocket(input: &str) -> u64 {
    let mut pocket = 0;
    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            let code = if c.is_ascii_uppercase() {
                1 << (26 + c as u8 - b'A')
            } else {
                1 << (c as u8 - b'a')
            };
            pocket |= code;
        }
    }
    pocket
}

fn part1(lines: &[String]) -> Result<String> {
    let mut sum = 0;
    for line in lines {
        let line = line.trim();
        let mid = line.len() / 2;

        let mut pocket1 = count_pocket(&line[..mid]);
        let mut pocket2 = count_pocket(&line[mid..]);

        for i in 1..=(26 * 2) {
            if (pocket1 & 1) != 0 && (pocket2 & 1) != 0 {
                sum += i;
            }
            pocket1 >>= 1;
            pocket2 >>= 1;
        }
    }
    Ok(format!("sum = {}", sum))
}

fn part2(lines: &[String]) -> Result<String> {
    let mut sum = 0;
    let mut idx = 0;
    let mut pocket = !(1 << 57);
    for line in lines {
        let line = line.trim();

        pocket &= count_pocket(&line);

        if idx == 2 {
            for i in 1..=(26 * 2) {
                if (pocket & 1) != 0 {
                    sum += i;
                    break;
                }
                pocket >>= 1;
            }
            pocket = !(1 << 57);
            idx = 0;
        } else {
            idx += 1;
        }
    }
    Ok(format!("sum = {}", sum))
}
