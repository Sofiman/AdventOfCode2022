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

#[derive(Clone, Copy, Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors
}

impl RPS {
    fn from(c: char) -> Self {
        use RPS::*;
        match c {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => unreachable!("Invalid character")
        }
    }

    fn score(&self) -> usize {
        use RPS::*;
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3
        }
    }
}

fn part1(lines: &[String]) -> Result<String> {
    let mut total = 0;
    for line in lines {
        let mut chars = line.chars();
        let opponent = RPS::from(chars.nth(0).unwrap());
        let me = RPS::from(chars.nth(1).unwrap());
        total += play_round(opponent, me);
    }
    Ok(format!("Total score: {}", total))
}

fn part2(lines: &[String]) -> Result<String> {
    let mut total = 0;
    for line in lines {
        let mut chars = line.chars();
        let opponent = RPS::from(chars.nth(0).unwrap());
        let me = RPS::from(chars.nth(1).unwrap());
        total += expect_round(opponent, me);
    }
    Ok(format!("Total score: {}", total))
}

fn play_round(opponent: RPS, me: RPS) -> usize {
    use RPS::*;
    let result = match (opponent, me) {
        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 6,
        _ => 0 // lost
    };
    return result + me.score();
}

fn expect_round(opponent: RPS, me: RPS) -> usize {
    use RPS::*;
    let result = match me {
        Rock => 0 + match opponent {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper
        }.score(), // lost
        Paper => 3 + opponent.score(), // draw
        Scissors => 6 + match opponent {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock
        }.score() // win
    };
    return result;
}
