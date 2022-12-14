use std::{fs::File, io::{Result, BufReader, BufRead}, cmp::Ordering};

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

enum Packet {
    Number(usize),
    List(Vec<Packet>)
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Packet::*;
        match self {
            Number(x) => write!(f, "{}", x),
            List(children) => {
                if children.is_empty() {
                    write!(f, "[]")
                } else {
                    write!(f, "[ {}", children[0])?;
                    for child in &children[1..] {
                        write!(f, ", {}", child)?;
                    }
                    write!(f, " ]")
                }
            }
        }
    }
}

fn parse_packet(line: &[char], mut i: usize, end: usize) -> Packet {
    use Packet::*;
    if i == end {
        panic!("err")
    }
    if line[i] == '[' {
        i += 1;
        let mut v = vec![];
        let mut j = i;
        while j < end && line[j] != ']' {
            if line[j] == ',' {
                v.push(parse_packet(line, i, j));
                i = j + 1;
            }
            j += 1;
        }
        if i != j {
            v.push(parse_packet(line, i, j));
        }
        List(v)
    }
    else {
        let l: String = line[i..end].iter().collect();
        let x: usize = l.parse().unwrap();
        Number(x)
    }
}

fn compare(left: Packet, right: Packet) -> Ordering {
    use Packet::*;
    match (left, right) {
        (Number(x), Number(y)) => x.cmp(&y),
        (List(a), List(b)) => {
            let la = a.len();
            let lb = b.len();
            for (p1, p2) in a.into_iter().zip(b.into_iter()) {
                let result = compare(p1, p2);
                if result != Ordering::Equal {
                    return result;
                }
            }
            return la.cmp(&lb);
        },
        (Number(x), List(a)) => compare(List(vec![Number(x)]), List(a)),
        (List(a), Number(x)) => compare(List(a), List(vec![Number(x)])),
    }
}

fn part1(lines: &[String]) -> Result<String> {
    let mut buf: Vec<&str> = vec![];
    let mut sum = 0;
    let mut i = 1;
    for line in lines {
        if line.trim().is_empty() {
            let left_chars: Vec<char> = buf[0].chars().collect();
            let left = parse_packet(&left_chars, 0, left_chars.len());
            let right_chars: Vec<char> = buf[1].chars().collect();
            let right = parse_packet(&right_chars, 0, right_chars.len());
            println!("a = {}", left);
            println!("b = {}", right);
            if compare(left, right) != Ordering::Less { // in order
                println!("=> {} is in \x1b[32mthe right\x1b[0m order", i);
                sum += i;
            } else {
                println!("=> {} is \x1b[31mnot in the right\x1b[0m order", i);
            }
            buf.clear();
            i += 1;
        } else {
            buf.push(line);
        }
    }
    if buf.len() != 0 {
            let left_chars: Vec<char> = buf[0].chars().collect();
            let left = parse_packet(&left_chars, 0, left_chars.len());
            let right_chars: Vec<char> = buf[1].chars().collect();
            let right = parse_packet(&right_chars, 0, right_chars.len());
            println!("a = {}", left);
            println!("b = {}", right);
            if compare(left, right) != Ordering::Less { // in order
                println!("=> {} is in \x1b[32mthe right\x1b[0m order", i);
                sum += i;
            } else {
                println!("=> {} is \x1b[31mnot in the right\x1b[0m order", i);
            }
    }

    Ok(format!("sum = {}", sum))
}

fn part2(lines: &[String]) -> Result<String> {
    todo!("Part 2")
}
