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

fn dist((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> u32 {
    (x1 - x2).abs() as u32 + (y1 - y2).abs() as u32
}

#[derive(Debug, Clone, Copy)]
enum CellKind {
    Void,
    Beacon,
    Sensor((i32, i32))
}

fn parse_reading(line: &str, map: &mut [CellKind], width: usize, mid: i32) {
    let (sensor_pos, beacon_pos) = line.split_once(": ").unwrap();
    let (x, y) = sensor_pos[10..].split_once(", ").unwrap();
    let x: i32 = x.strip_prefix("x=").unwrap().parse().unwrap();
    let y: i32 = y.strip_prefix("y=").unwrap().parse().unwrap();

    let (closest_x, closest_y) = beacon_pos[21..].split_once(", ").unwrap();
    let closest_x: i32 = closest_x.strip_prefix("x=").unwrap().parse().unwrap();
    let closest_y: i32 = closest_y.strip_prefix("y=").unwrap().parse().unwrap();

    map[(y as usize) * width + (x + mid) as usize] = CellKind::Sensor((closest_x, closest_y));
    map[(closest_y as usize) * width + (closest_x + mid) as usize] = CellKind::Beacon;
}

fn part1(lines: &[String]) -> Result<String> {
    let width = 80;
    let height = 30;
    let mut map = vec![CellKind::Void; width * height];
    for line in lines {
        parse_reading(line, &mut map, width, width as i32 / 2);
    }

    for i in 0..height {
        for j in 0..width {
            match map[i * width + j] {
                CellKind::Void => print!("."),
                CellKind::Sensor(_) => print!("S"),
                CellKind::Beacon => print!("B"),
            }
        }
        println!()
    }
    todo!("Part 1")
}

fn part2(lines: &[String]) -> Result<String> {
    todo!("Part 2")
}
