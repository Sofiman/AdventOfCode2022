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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

fn drop_sand(map: &mut [Unit], width: usize, height: usize) -> bool {
    let (mut x, mut y) = (25, 0);
    while y < height - 1 {
        let below = map[(y + 1) * width + x];
        if below != Unit::Void {
            if x > 0 && map[(y + 1) * width + x - 1] == Unit::Void {
                x -= 1;
            } else if x < width - 1 && map[(y + 1) * width + x + 1] == Unit::Void {
                x += 1;
            } else {
                break;
            }
        } else {
            y += 1;
        }
    }
    if y != height - 1 {
        map[y * width + x] = Unit::Sand;
        return false;
    }
    true
}

fn part1(lines: &[String]) -> Result<String> {
    let height = 180;
    let mut map = vec![Unit::Void; 50 * height];
    for line in lines {
        draw_line(line, &mut map, 425, 50);
    }

    let mut i = 0;
    while !drop_sand(&mut map, 50, height) {
        i += 1;
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

    Ok(format!("{}", i))
}

fn part2(lines: &[String]) -> Result<String> {
    todo!("Part 2")
}
