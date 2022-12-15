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

#[derive(Debug, Clone)]
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

fn parse_packet(line: &[char], i: usize, end: usize) -> (Option<Packet>, usize) {
    use Packet::*;
    if line[i] == '[' {
        let mut v = vec![];
        let mut j = i + 1;
        while j < end {
            let (e, right) = parse_packet(line, j, end);
            if let Some(e) = e {
                v.push(e);
            }
            j = right + 1;
            if line[right] == ']' {
                break;
            }
        }
        (Some(List(v)), j)
    }
    else {
        let mut right = i;
        while right < end && line[right] != ',' && line[right] != ']' {
            right += 1;
        }
        if right - i < 1 {
            return (None, right)
        }
        let l: String = line[i..right].iter().collect();
        let x: usize = l.parse().unwrap();
        (Some(Number(x)), right)
    }
}

fn compare(left: &Packet, right: &Packet) -> Ordering {
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
        (Number(_), List(_)) => compare(&List(vec![left.clone()]), right),
        (List(_), Number(_)) => compare(left, &List(vec![right.clone()])),
    }
}

fn part1(lines: &[String]) -> Result<String> {
    let mut buf: Vec<&str> = vec![];
    let mut sum = 0;
    let mut i = 1;
    for line in lines {
        if line.trim().is_empty() {
            let left_chars: Vec<char> = buf[0].chars().collect();
            let (left, _) = parse_packet(&left_chars, 0, left_chars.len());
            let right_chars: Vec<char> = buf[1].chars().collect();
            let (right, _) = parse_packet(&right_chars, 0, right_chars.len());
            if compare(&left.unwrap(), &right.unwrap()) != Ordering::Greater { // in order
                println!("=> {} is in \x1b[32mthe right\x1b[0m order", i);
                sum += i;
            } else {
                println!("=> {} is \x1b[31mnot in the right\x1b[0m order", i);
            }
            println!();
            buf.clear();
            i += 1;
        } else {
            buf.push(line);
        }
    }
    if buf.len() != 0 {
        let left_chars: Vec<char> = buf[0].chars().collect();
        let (left, _) = parse_packet(&left_chars, 0, left_chars.len());
        let right_chars: Vec<char> = buf[1].chars().collect();
        let (right, _) = parse_packet(&right_chars, 0, right_chars.len());
        if compare(&left.unwrap(), &right.unwrap()) != Ordering::Greater { // in order
            println!("=> {} is in \x1b[32mthe right\x1b[0m order", i);
            sum += i;
        } else {
            println!("=> {} is \x1b[31mnot in the right\x1b[0m order", i);
        }
    }

    Ok(format!("sum = {}", sum))
}

fn part2(lines: &[String]) -> Result<String> {
    use Packet::*;
    let mut all: Vec<Packet> = vec![
        List(vec![List(vec![Number(2)])]),
        List(vec![List(vec![Number(6)])])
    ];
    for line in lines {
        if !line.trim().is_empty() {
            let chars: Vec<char> = line.chars().collect();
            let (packet, _) = parse_packet(&chars, 0, chars.len());
            all.push(packet.unwrap());
        }
    }
    all.sort_by(|a, b| compare(a, b));

    let mut found = None;
    for (i, p) in all.iter().enumerate() {
        if let List(top) = p {
            if top.len() == 1 {
                if let List(down) = &top[0] {
                    if down.len() == 1 {
                        if let Number(x) = down[0] {
                            match x {
                                2 => found = Some(i + 1),
                                6 => {
                                    found = found.map(|x| x * (i + 1));
                                    break;
                                },
                                _ => ()
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(format!("key: {:?}", found))
}
