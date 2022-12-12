use std::{fs::File, io::{Result, BufReader, BufRead}};
use std::fmt::Write;

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

struct CPU {
    x: i32
}

impl CPU {
    fn new() -> Self {
        Self { x: 1 }
    }

    fn execute_op(&mut self, intr: &str) -> i32 {
        if intr.starts_with("noop") {
            1
        } else {
            let (_, d) = intr.split_once(" ").unwrap();
            let d: i32 = d.parse().unwrap();
            self.x += d;
            2
        }
    }
}

fn part1(lines: &[String]) -> Result<String> {
    let watched_cycles = [20, 60, 100, 140, 180, 220];
    let mut watched_id = 0;
    let mut cpu = CPU::new();
    let mut sum = 0;
    let mut nb_cycle = 0;
    for intr in lines {
        let prev_x = cpu.x;
        nb_cycle += cpu.execute_op(intr);
        if nb_cycle >= watched_cycles[watched_id] {
            if nb_cycle == watched_cycles[watched_id] {
                sum += watched_cycles[watched_id] * prev_x;
            } else {
                sum += watched_cycles[watched_id] * cpu.x;
            }
            watched_id += 1;
            if watched_id == watched_cycles.len() {
                break;
            }
        }
    }
    Ok(format!("total = {}", sum))
}

fn part2(lines: &[String]) -> Result<String> {
    let mut cpu = CPU::new();
    let mut screen = vec![false; 40 * 6];
    let mut cycle: usize = 0;

    for intr in lines {
        let prev_x = cpu.x;
        if cpu.execute_op(intr) == 2 {
            let mut pixel = cycle as i32 % 40;
            if (prev_x - pixel).abs() < 2 {
                screen[cycle] = true;
            }
            pixel += 1;
            cycle += 1;
            if (prev_x - pixel).abs() < 2 {
                screen[cycle] = true;
            }
            cycle += 1;
            continue;
        }
        let pixel = cycle as i32 % 40;
        if (cpu.x - pixel).abs() < 2 {
            screen[cycle] = true;
        }
        cycle += 1;
    }

    let mut f = String::new();
    for i in 0..6 {
        for j in 0..40 {
            if screen[i * 40 + j] {
                write!(f, "#").unwrap();
            } else {
                write!(f, ".").unwrap();
            }
        }
        writeln!(f).unwrap();
    }
    Ok(f)
}
