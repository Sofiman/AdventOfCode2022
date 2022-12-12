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

fn part1(lines: &[String]) -> Result<String> {
    let width = lines[0].len();
    let mut grid = vec![0; width * lines.len()];
    for (i, line) in lines.iter().enumerate() {
        let mut j = 0;
        for c in line.chars() {
            grid[i * width + j] = c.to_digit(10).unwrap();
            j += 1;
        }
    }
    let mut visible = vec![false; grid.len()];
    let mut count = 0;

    for i in 0..lines.len() {
        for j in 0..width {
            let height = grid[i * width + j];
            let mut k = j;
            // to the left
            while k > 0 && grid[i * width + k - 1] < height {
                k -= 1;
            }
            if k == 0 {
                visible[i * width + j] = true;
                count += 1;
                continue;
            }

            k = j + 1;
            while k < width && grid[i * width + k] < height {
                k += 1;
            }
            if k == width {
                visible[i * width + j] = true;
                count += 1;
                continue;
            }

            k = i;
            while k > 0 && grid[(k - 1) * width + j] < height {
                k -= 1;
            }
            if k == 0 {
                visible[i * width + j] = true;
                count += 1;
                continue;
            }

            k = i + 1;
            while k < lines.len() && grid[k * width + j] < height {
                k += 1;
            }
            if k == lines.len() {
                visible[i * width + j] = true;
                count += 1;
            }
        }
    }

    let total = visible.iter().fold(0, |acc, &x| acc + x as usize);
    for i in 0..lines.len() {
        for j in 0..width {
            if visible[i * width + j] {
                print!("\x1b[32m{}\x1b[0m", grid[i * width + j] as u8);
            } else {
                print!("\x1b[2m{}\x1b[0m", grid[i * width + j] as u8);
            }
        }
        println!();
    }
    Ok(format!("visible: {}", total))
}

fn part2(lines: &[String]) -> Result<String> {
    let width = lines[0].len();
    let mut grid = vec![0; width * lines.len()];
    for (i, line) in lines.iter().enumerate() {
        let mut j = 0;
        for c in line.chars() {
            grid[i * width + j] = c.to_digit(10).unwrap();
            j += 1;
        }
    }
    let mut max_score = 0;

    for i in 0..lines.len() {
        for j in 0..width {
            let height = grid[i * width + j];
            let mut k = j;

            k = i;
            while k > 0 && grid[(k - 1) * width + j] < height {
                k -= 1;
            }
            if k != 0 && grid[(k - 1) * width + j] == height {
                k -= 1;
            }
            let top = i - k + 1;

            k = j;
            while k > 0 && grid[i * width + k - 1] < height {
                k -= 1;
            }
            if k != 0 && grid[i * width + k - 1] == height {
                k -= 1;
            }
            let left = j - k + 1;

            k = j;
            while k < width && grid[i * width + k] < height {
                k += 1;
            }
            if k != width && grid[i * width + k] == height {
                k += 1;
            }
            let right = k - j;

            k = i;
            while k < lines.len() && grid[k * width + j] < height {
                k += 1;
            }
            if k != lines.len() && grid[k * width + j] == height {
                k += 1;
            }
            let bottom = k - i;
            println!("grid[{}, {}]: {} => {} * {} * {} * {} = {}", i, j,
                     height, top, left, right,
                     bottom, top * left * right * bottom);
        }
    }

    Ok(format!("highest score: {}", max_score))
}
