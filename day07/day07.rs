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

#[derive(Debug, Clone)]
enum Command {
    Unknown,
    ListDirs,
    ChangeDir(String),
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    files: Vec<(String, usize)>,
    dirs: Vec<Box<Directory>>
}

#[derive(Debug, Clone)]
enum Action {
    None,
    PushTopLevel(String),
    PopRoot,
    PopTopLevel
}

fn parse_output(cmd: &Command, buf: &[&str], current_dir: &mut Directory)
    -> Action {
    use Command::*;
    use Action::*;
    match cmd {
        ChangeDir(target) => match target.as_str() {
            "/"  => PopRoot,
            ".." => PopTopLevel,
            name => PushTopLevel(name.to_string())
        },
        ListDirs => {
            for line in buf {
                if line.starts_with("dir") {
                    current_dir.dirs.push(Box::new(Directory {
                        name: line[4..].to_string(),
                        files: vec![],
                        dirs: vec![]
                    }));
                } else {
                    let (size, name) = line.split_once(" ").unwrap();
                    let size: usize = size.parse().unwrap();
                    current_dir.files.push((name.to_string(), size));
                }
            }
            None
        },
        Unknown => None
    }
}

fn parse_cmd(line: &str) -> Command {
    if let Some((cmd, arg)) = line.split_once(" ") {
        match cmd {
            "cd" => Command::ChangeDir(arg.to_string()),
            _ => Command::Unknown
        }
    } else {
        match line {
            "ls" => Command::ListDirs,
            _ => Command::Unknown
        }
    }
}

fn part1(lines: &[String]) -> Result<String> {
    let root = Directory {
        name: "/".to_string(), files: vec![], dirs: vec![]
    };
    let mut current_cmd = Command::Unknown;
    let mut path: Vec<Box<Directory>> = vec![Box::new(root)];
    let mut output_buf = vec![];

    for line in lines {
        let last = path.len() - 1;
        if line.starts_with("$ ") {
            use Action::*;
            match parse_output(&current_cmd, &output_buf, &mut path[last]) {
                PopRoot => path.truncate(1),
                PopTopLevel => { path.pop(); },
                PushTopLevel(target) => {
                    let to_add = path[last].dirs.iter()
                        .find(|x| *x.name == target).unwrap();
                    path.push(to_add.clone());
                },
                None => ()
            }
            output_buf.clear();

            current_cmd = parse_cmd(&line[2..]);
        } else if !matches!(current_cmd, Command::Unknown) {
            output_buf.push(line);
        }
    }

    Ok(format!("test"))
}

fn part2(lines: &[String]) -> Result<String> {
    todo!("Part 2")
}
