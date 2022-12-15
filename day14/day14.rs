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

#[derive(Debug, Clone, Copy)]
enum Unit {
    Void,
    Wall,
    Sand
}

fn draw_line(line: &str, map: &mut [Unit], mid: usize, width: usize) {
    let mut prev = None;
    for point in line.split(" -> ") {
        let (x, y) = point.split_once(",").unwrap();
        let x: usize = x.parse().unwrap();
        let y: usize = y.parse().unwrap();
        if let Some((from_x, from_y)) = prev {
            if from_x == x {
                // vertical
                if from_y > y {
                    for i in 0..=(from_y - y) {
                        map[(y + i) * width + x - mid] = Unit::Wall;
                    }
                } else {
                    for i in 0..=(y - from_y) {
                        map[(from_y + i) * width + x - mid] = Unit::Wall;
                    }
                }
            } else if from_y == y {
                // horizontal
                if from_x > x {
                    for i in 0..=(from_x - x) {
                        map[y * width + x + i - mid] = Unit::Wall;
                    }
                } else {
                    for i in 0..=(x - from_x) {
                        map[y * width + from_x + i - mid] = Unit::Wall;
                    }
                }
            }
        }

        prev.replace((x, y));
    }
}

fn part1(lines: &[String]) -> Result<String> {
    let height = 11;
    let mut map = vec![Unit::Void; 50 * height];
    for line in lines {
        draw_line(line, &mut map, 425, 50);
    }

    for i in 0..height {
        for j in 0..50 {
            match map[i * 50 + j] {
                Unit::Sand => print!("O"),
                Unit::Wall => print!("#"),
                Unit::Void => print!("."),
            }
        }
        println!();
    }
    todo!("Part 1")
}

fn part2(lines: &[String]) -> Result<String> {
    todo!("Part 2")
}
