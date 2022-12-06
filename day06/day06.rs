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

fn is_distinct(packet: &[char]) -> bool {
    for i in 0..packet.len() {
        for j in (i + 1)..packet.len() {
            if packet[i] == packet[j] {
                return false;
            }
        }
    }
    true
}

fn part1(lines: &[String]) -> Result<String> {
    let mut starts = vec![];
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let mut idx = 0;
        for packet in chars.windows(4) {
            if is_distinct(packet) {
                starts.push(idx + 4);
                break;
            }
            idx += 1;
        }
    }
    Ok(format!("{:?}", starts))
}

fn part2(lines: &[String]) -> Result<String> {
    let mut starts = vec![];
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        let mut idx = 0;
        for packet in chars.windows(14) {
            if is_distinct(packet) {
                starts.push(idx + 14);
                break;
            }
            idx += 1;
        }
    }
    Ok(format!("{:?}", starts))
}
