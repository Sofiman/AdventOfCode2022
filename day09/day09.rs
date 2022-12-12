use std::{fs::File, io::{Result, BufReader, BufRead}};
use std::collections::HashSet;

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
    let mut head: (i32, i32) = (0, 5);
    let mut tail: (i32, i32) = (0, 5);
    let mut visited = HashSet::new();
    visited.insert(tail);
    for line in lines {
        let (kind, amount) = line.split_once(" ").unwrap();
        let mut amount: u16 = amount.parse().unwrap();
        //println!("== {} ==", line);
        while amount > 0 {
            match kind {
                "U" => head.1 -= 1,
                "L" => head.0 -= 1,
                "D" => head.1 += 1,
                "R" => head.0 += 1,
                _ => unreachable!("Unknown direction")
            }
            let dx = head.0 - tail.0;
            let dy = head.1 - tail.1;
            if dx.abs() > 1 || dy.abs() > 1 {
                if dx != 0 {
                    tail.0 += dx / dx.abs();
                }
                if dy != 0 {
                    tail.1 += dy / dy.abs();
                }
                visited.insert(tail);
            }

            /*
            for i in 0..6 {
                for j in 0..6 {
                    if head.0 == j && head.1 == i {
                        print!("H");
                    } else if tail.0 == j && tail.1 == i {
                        print!("T");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            println!();
            */

            amount -= 1;
        }
    }
    Ok(format!("total visited: {}", visited.len()))
}

fn part2(lines: &[String]) -> Result<String> {
    let mut segments: Vec<(i32, i32)> = vec![(0, 5); 10];
    let mut visited = HashSet::new();
    visited.insert(segments[0]);
    for line in lines {
        let (kind, amount) = line.split_once(" ").unwrap();
        let mut amount: u16 = amount.parse().unwrap();
        //println!("== {} ==", line);
        while amount > 0 {
            let mut head = &mut segments[0];
            match kind {
                "U" => head.1 -= 1,
                "L" => head.0 -= 1,
                "D" => head.1 += 1,
                "R" => head.0 += 1,
                _ => unreachable!("Unknown direction")
            }
            for i in 1..segments.len() {
                let seg = segments[i - 1];
                let mut tail = &mut segments[i];
                let dx = seg.0 - tail.0;
                let dy = seg.1 - tail.1;
                if dx.abs() > 1 || dy.abs() > 1 {
                    if dx != 0 {
                        tail.0 += dx / dx.abs();
                    }
                    if dy != 0 {
                        tail.1 += dy / dy.abs();
                    }
                }
            }
            visited.insert(segments[segments.len() - 1]);

            /*
            for i in 0..6 {
                for j in 0..6 {
                    if segments[0].0 == j && segments[0].1 == i {
                        print!("H");
                    } else {
                        let mut found = false;
                        for (k, seg) in segments[1..].iter().enumerate() {
                            if seg.0 == j && seg.1 == i {
                                found = true;
                                print!("{}", k + 1);
                                break;
                            }
                        }
                        if !found {
                            print!(".");
                        }
                    }
                }
                println!();
            }
            println!();
            */

            amount -= 1;
        }
    }
    Ok(format!("total visited: {}", visited.len()))
}
