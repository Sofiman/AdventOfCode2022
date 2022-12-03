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
    let mut cur = 0;
    let mut top = (0, 0);
    let mut idx = 1;
    for line in lines {
        let line = line.trim();

        if line.len() == 0 {
            if cur > top.0 {
                top = (cur, idx);
            }
            cur = 0;
            idx += 1;
        } else {
            cur += line.parse().unwrap_or(0);
        }
    }
    Ok(format!("elfe {} brought {} calories", top.1, top.0))
}

fn part2(lines: &[String]) -> Result<String> {
    let mut top = vec![(0,0)];
    let mut idx = 1;
    for line in lines {
        let line = line.trim();

        if line.len() == 0 {
            top.push((0, idx));
            idx += 1;
        } else {
            let pos = top.len() - 1;
            let head = &mut top[pos];
            head.0 += line.parse().unwrap_or(0);
        }
    }
    top.sort();

    let mut total = 0;
    println!("Top 3:");
    for (sum, id) in &top[(top.len() - 3)..] {
        println!("- {} calories by elfe {}", sum, id);
        total += sum;
    }
    Ok(format!("... brought a total of {} calories", total))
}
