use std::{fs::File, io::{BufReader, BufRead}, str::FromStr, ops::{Add, Neg}};

use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
    }

    errors {
        InvalidDirection(direction: String) {
            description("invalid direction"),
            display("invalid direction: {}", direction),
        }
    }
}

fn main() -> Result<()> {
    let input = File::open("input.txt")?;
    let buffered = BufReader::new(input);

    let position = buffered
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<&str>>();
            let direction = Direction::from_str(parts[0]).unwrap();
            let distance = parts[1].parse::<usize>().unwrap();

            (direction, distance)
        })
        .map(Command::from)
        .fold(Position::default(), |acc, cur| acc + cur);


    println!("Final vector: {}", position.final_position());
    Ok(())
}


#[derive(Debug, Default)]
struct Position {
    horizontal: isize,
    depth: isize,
    aim: isize
}

impl Position {
    pub fn final_position(&self) -> isize {
        self.horizontal * self.depth
    }
}

impl Add<Command> for Position {
    type Output = Self;

    fn add(self, rhs: Command) -> Self::Output {
        match rhs.direction() {
            Direction::Forward => {
                Self {
                    horizontal: self.horizontal + rhs.distance(),
                    depth: self.depth + (self.aim * rhs.distance()),
                    ..self
                }
            },
            _ => {
                Self {
                    aim: self.aim + rhs.distance(),
                    ..self
                }
            }
        }
    }
}

struct Command {
    direction: Direction,
    distance: usize
}

impl Command {
    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn distance(&self) -> isize {
        match self.direction {
            Direction::Up => (self.distance as isize).neg(),
            _ => self.distance as isize
        }
    }
}

impl From<(Direction, usize)> for Command {
    fn from((direction, distance): (Direction, usize)) -> Self {
        Self {
            direction,
            distance
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Forward,
    Up,
    Down
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err(ErrorKind::InvalidDirection(s.to_string()).into())
        }
    }
}