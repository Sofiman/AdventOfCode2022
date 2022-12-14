use std::{fs::File, io::{Result, BufReader, BufRead}};
use std::collections::VecDeque;

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

fn can_move_to(from: u32, to: u32) -> bool {
    (to > from && to - from <= 1) || to <= from
}

fn shortest_path(graph: &[u32], width: usize, src: usize) -> Vec<Option<usize>> {
    let mut to_process: VecDeque<usize> = VecDeque::new();
    let mut marks = vec![None; graph.len()];
    let h = graph.len() / width;

    marks[src] = Some(graph.len());
    to_process.push_front(src);

    while !to_process.is_empty() {
        let v = to_process.pop_front().unwrap();
        let col = v % width;
        let row = v / width;
        let height = graph[v];

        if col > 0 && can_move_to(height, graph[row * width + col - 1]) {
            let idx = row * width + col - 1;
            if let None = marks[idx] {
                marks[idx] = Some(v);
                to_process.push_back(idx);
            }
        }

        if col < width - 1 && can_move_to(height, graph[row * width + col + 1]) {
            let idx = row * width + col + 1;
            if let None = marks[idx] {
                marks[idx] = Some(v);
                to_process.push_back(idx);
            }
        }

        if row > 0 && can_move_to(height, graph[(row - 1) * width + col]) {
            let idx = (row - 1) * width + col;
            if let None = marks[idx] {
                marks[idx] = Some(v);
                to_process.push_back(idx);
            }
        }

        if row < h - 1 && can_move_to(height,
            graph[(row + 1) * width + col]) {
            let idx = (row + 1) * width + col;
            if let None = marks[idx] {
                marks[idx] = Some(v);
                to_process.push_back(idx);
            }
        }
    }
    marks
}

fn part1(lines: &[String]) -> Result<String> {
    let width = lines[0].len();
    let mut height_map = vec![0; lines.len() * width];
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (i, line) in lines.iter().enumerate() {
        for (j, h) in line.chars().enumerate() {
            match h {
                'S' => {
                    start = (j, i);
                    height_map[i * width + j] = 0; // a
                },
                'E' => {
                    end = (j, i);
                    height_map[i * width + j] = 25; // z
                },
                'a'..='z' => height_map[i * width + j] = h as u32 - 'a' as u32,
                _ => unreachable!("Unsupported height")
            }
        }
    }

    let marks = shortest_path(&height_map[..], width, start.0 + start.1 * width);

    let mut path = vec![];
    let mut dst = end.0 + end.1 * width;
    if let Some(parent) = marks[dst] {
        dst = parent;
        while dst != height_map.len() {
            path.push(dst);
            if let Some(parent) = marks[dst] {
                dst = parent;
            }
        }
    }

    for &i in path.iter() {
        height_map[i] = 26;
    }

    println!();
    for i in 0..lines.len() {
        for j in 0..width {
            match height_map[i * width + j] {
                0..=25 => {
                    let c = char::from_u32('a' as u32 + height_map[i * width + j]).unwrap();
                    if matches!(marks[i * width + j], None) {
                        print!("\x1b[2m{}\x1b[0m", c);
                    } else {
                        print!("{}", c);
                    }
                },
                26 => print!("\x1b[33m.\x1b[0m"),
                27.. => unreachable!()
            }
        }
        println!()
    }
    println!();

    Ok(format!("length = {}", path.len()))
}

fn part2(lines: &[String]) -> Result<String> {
    todo!("Part 2")
}
