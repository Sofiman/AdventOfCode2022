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
    Rock,
    Sand
}

fn draw_line(line: &str, map: &mut [Unit], mid: usize, width: usize,
             max_y: &mut usize) {
    let mut prev = None;
    for point in line.split(" -> ") {
        let (x, y) = point.split_once(",").unwrap();
        let x: usize = x.parse().unwrap();
        let y: usize = y.parse().unwrap();
        if y > *max_y {
            *max_y = y;
        }
        if let Some((from_x, from_y)) = prev {
            if from_x == x {
                // vertical
                if from_y > y {
                    for i in 0..=(from_y - y) {
                        map[(y + i) * width + x - mid] = Unit::Rock;
                    }
                } else {
                    for i in 0..=(y - from_y) {
                        map[(from_y + i) * width + x - mid] = Unit::Rock;
                    }
                }
            } else if from_y == y {
                // horizontal
                if from_x > x {
                    for i in 0..=(from_x - x) {
                        map[y * width + x + i - mid] = Unit::Rock;
                    }
                } else {
                    for i in 0..=(x - from_x) {
                        map[y * width + from_x + i - mid] = Unit::Rock;
                    }
                }
            }
        }

        prev.replace((x, y));
    }
}

fn drop_sand(map: &mut [Unit], width: usize, height: usize)
    -> Option<(usize, usize)> {
    let (mut x, mut y) = (width / 2, 0);
    while y < height - 1 {
        let idx = (y + 1) * width + x;
        if map[idx] != Unit::Void {
            if x > 0 && map[idx - 1] == Unit::Void {
                x -= 1;
            } else if x < width - 1 && map[idx + 1] == Unit::Void {
                x += 1;
            } else {
                break;
            }
        }
        y += 1;
    }
    if y != height - 1 {
        map[y * width + x] = Unit::Sand;
        return Some((x, y));
    }
    None
}

fn part1(lines: &[String]) -> Result<String> {
    let width = 100;
    let height = 170;
    let mut map = vec![Unit::Void; width * height];
    let mut max_y = 0;
    for line in lines {
        draw_line(line, &mut map, 500 - (width / 2), width, &mut max_y);
    }

    let mut i: usize = 0;
    while matches!(drop_sand(&mut map, width, height), Some(_)) {
        i += 1;
    }

    for i in 0..height {
        for j in 0..width {
            match map[i * width + j] {
                Unit::Sand => print!("O"),
                Unit::Rock => print!("#"),
                Unit::Void => print!("."),
            }
        }
        println!();
    }

    Ok(format!("{}", i))
}

fn part2(lines: &[String]) -> Result<String> {
    let width = 350;
    let height = 170;
    let mut map = vec![Unit::Void; width * height];
    let mut max_y = 0;
    for line in lines {
        draw_line(line, &mut map, 500 - (width / 2), width, &mut max_y);
    }
    max_y += 2;

    for i in 0..width {
        map[max_y * width + i] = Unit::Rock;
    }

    let mut i: usize = 0;
    loop {
        let (x, y) = drop_sand(&mut map, width, height).unwrap();
        i += 1;
        if x == width / 2 && y == 0 {
            break;
        }
    }

    for i in 0..height {
        for j in 0..width {
            match map[i * width + j] {
                Unit::Sand => print!("O"),
                Unit::Rock => print!("#"),
                Unit::Void => print!("."),
            }
        }
        println!();
    }

    Ok(format!("{}", i))
}
