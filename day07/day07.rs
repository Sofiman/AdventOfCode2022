use std::{fs::File, io::{Result, BufReader, BufRead}};
use std::collections::HashMap;

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
    sub_dirs: Vec<String>
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

fn size(dir: &Directory, map: &HashMap<String, Directory>) -> usize {
    let mut sub_total = 0;
    for (name, size) in &dir.files {
        sub_total += size;
    }
    for subdir in &dir.sub_dirs {
        sub_total += size(&map[subdir], map);
    }
    sub_total
}

fn part1(lines: &[String]) -> Result<String> {
    use Command::*;
    let mut fs: HashMap<String, Directory> = HashMap::new();
    let mut path: String = "/".to_string();
    fs.insert(path.clone(), Directory {
        name: "/".to_string(), files: vec![], sub_dirs: vec![]
    });

    let mut current_cmd = Command::Unknown;
    let mut output_buf: Vec<&str> = vec![];
    for line in lines {
        if line.starts_with("$ ") {
            match current_cmd {
                ChangeDir(target) => match target.as_str() {
                    "/"  => path = "/".to_string(),
                    ".." => {
                        if let Some((parent, _)) = path.rsplit_once("/") {
                            path = parent.to_string();
                        }
                    },
                    name => {
                        path += "/";
                        path += name;
                    },
                },
                ListDirs => {
                    for line in &output_buf {
                        if line.starts_with("dir") {
                            let new_path = path.clone() + "/" + &line[4..];
                            if let Some(dir) = fs.get_mut(&path) {
                                dir.sub_dirs.push(new_path.clone());
                            }
                            fs.insert(new_path, Directory {
                                name: line[4..].to_string(),
                                files: vec![], sub_dirs: vec![]
                            });
                        } else {
                            let (size, name) = line.split_once(" ").unwrap();
                            let size: usize = size.parse().unwrap();
                            if let Some(dir) = fs.get_mut(&path) {
                                dir.files.push((name.to_string(), size));
                            }
                        }
                    }
                },
                Unknown => ()
            };
            output_buf.clear();
            current_cmd = parse_cmd(&line[2..]);
        } else if !matches!(current_cmd, Command::Unknown) {
            output_buf.push(line);
        }
    }
    if matches!(current_cmd, Command::ListDirs) {
        for line in &output_buf {
            if line.starts_with("dir") {
                let new_path = path.clone() + "/" + &line[4..];
                if let Some(dir) = fs.get_mut(&path) {
                    dir.sub_dirs.push(new_path.clone());
                }
                fs.insert(new_path, Directory {
                    name: line[4..].to_string(),
                    files: vec![], sub_dirs: vec![]
                });
            } else {
                let (size, name) = line.split_once(" ").unwrap();
                let size: usize = size.parse().unwrap();
                if let Some(dir) = fs.get_mut(&path) {
                    dir.files.push((name.to_string(), size));
                }
            }
        }
    }

    let mut total = 0;
    for (key, val) in &fs {
        let sub_total = size(val, &fs);
        if sub_total < 100000 {
            total += sub_total;
        }
    }

    Ok(format!("{}", total))
}

fn part2(lines: &[String]) -> Result<String> {
    use Command::*;
    let mut fs: HashMap<String, Directory> = HashMap::new();
    let mut path: String = "/".to_string();
    fs.insert(path.clone(), Directory {
        name: "/".to_string(), files: vec![], sub_dirs: vec![]
    });

    let mut current_cmd = Command::Unknown;
    let mut output_buf: Vec<&str> = vec![];
    for line in lines {
        if line.starts_with("$ ") {
            match current_cmd {
                ChangeDir(target) => match target.as_str() {
                    "/"  => path = "/".to_string(),
                    ".." => {
                        if let Some((parent, _)) = path.rsplit_once("/") {
                            path = parent.to_string();
                        }
                    },
                    name => {
                        path += "/";
                        path += name;
                    },
                },
                ListDirs => {
                    for line in &output_buf {
                        if line.starts_with("dir") {
                            let new_path = path.clone() + "/" + &line[4..];
                            if let Some(dir) = fs.get_mut(&path) {
                                dir.sub_dirs.push(new_path.clone());
                            }
                            fs.insert(new_path, Directory {
                                name: line[4..].to_string(),
                                files: vec![], sub_dirs: vec![]
                            });
                        } else {
                            let (size, name) = line.split_once(" ").unwrap();
                            let size: usize = size.parse().unwrap();
                            if let Some(dir) = fs.get_mut(&path) {
                                dir.files.push((name.to_string(), size));
                            }
                        }
                    }
                },
                Unknown => ()
            };
            output_buf.clear();
            current_cmd = parse_cmd(&line[2..]);
        } else if !matches!(current_cmd, Command::Unknown) {
            output_buf.push(line);
        }
    }
    if matches!(current_cmd, Command::ListDirs) {
        for line in &output_buf {
            if line.starts_with("dir") {
                let new_path = path.clone() + "/" + &line[4..];
                if let Some(dir) = fs.get_mut(&path) {
                    dir.sub_dirs.push(new_path.clone());
                }
                fs.insert(new_path, Directory {
                    name: line[4..].to_string(),
                    files: vec![], sub_dirs: vec![]
                });
            } else {
                let (size, name) = line.split_once(" ").unwrap();
                let size: usize = size.parse().unwrap();
                if let Some(dir) = fs.get_mut(&path) {
                    dir.files.push((name.to_string(), size));
                }
            }
        }
    }

    let mut free = 70000000 - size(&fs["/"], &fs);
    if free >= 30000000 {
        return Ok(format!("free: {}", free))
    }
    free = 30000000 - free;
    println!("free space: {}", free);
    let mut min = 70000000;
    for (key, val) in &fs {
        let sub_total = size(val, &fs);
        if sub_total >= free && sub_total < min {
            min = sub_total;
        }
    }

    Ok(format!("{}", min))
}
