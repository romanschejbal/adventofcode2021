use std::{
    io::{stdin, BufRead},
    num::ParseIntError,
    str::FromStr,
};

struct Command(String, i32);

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut parts = line.split(" ");
        Ok(Self(
            parts.next().unwrap().to_owned(),
            parts.next().unwrap().parse::<i32>()?,
        ))
    }
}

fn main() {
    let commands: Vec<_> = stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<Command>().unwrap())
        .collect();

    let (mut position, mut depth) = (0, 0);
    for Command(command, units) in commands.iter() {
        match &command[..] {
            "forward" => position += units,
            "down" => depth += units,
            "up" => depth -= units,
            _ => unreachable!(),
        }
    }

    println!("Part 1: {}", position * depth);

    let (mut position, mut depth, mut aim) = (0, 0, 0);
    for Command(command, units) in commands.iter() {
        match &command[..] {
            "forward" => {
                position += units;
                depth += aim * units;
            }
            "down" => aim += units,
            "up" => aim -= units,
            _ => unreachable!(),
        }
    }

    println!("Part 2: {}", position * depth);
}
