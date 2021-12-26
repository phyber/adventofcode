// day
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{
    self,
    prelude::*,
    BufReader,
};

// CLI arguments
type Args = Vec<String>;

#[derive(Debug)]
enum Direction {
    Down(usize),
    Forward(usize),
    Up(usize),
}

impl<S> From<S> for Direction
where S: AsRef<str> {
    fn from(s: S) -> Self {
        let frags: Vec<&str> = s.as_ref().splitn(2, ' ').collect();
        let direction = frags[0];
        let num = frags[1].parse().expect("valid integer");

        match direction {
            "down" => Self::Down(num),
            "forward" => Self::Forward(num),
            "up" => Self::Up(num),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Directions(Vec<Direction>);

impl<S> From<S> for Directions
where S: AsRef<str> {
    fn from(s: S) -> Self {
        let mut directions = Vec::new();

        for line in s.as_ref().lines() {
            let direction = Direction::from(&line);

            directions.push(direction);
        }

        Self(directions)
    }
}

// Get an input reader
fn input_reader(
    args: Args,
) -> Result<BufReader<Box<dyn io::Read>>, Box<dyn Error>> {
    // Either read from the given file or stdin
    let input: Box<dyn io::Read> = if args.len() > 1 {
        let filename = &args[1];
        let fh = File::open(filename).unwrap();
        Box::new(fh)
    }
    else {
        let stdin = io::stdin();
        Box::new(stdin)
    };

    let reader = BufReader::new(input);

    Ok(reader)
}

fn part_one(directions: &Directions) {
    let mut hpos = 0;
    let mut depth = 0;

    for direction in directions.0.iter() {
        match direction {
            Direction::Down(num) => depth += num,
            Direction::Forward(num) => hpos += num,
            Direction::Up(num) => depth -= num,
        }
    }

    println!("H: {}, D: {}", hpos, depth);
    println!("Part 1: {}", hpos * depth);
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = env::args().collect();

    // Get the input
    let mut buffer = String::new();
    let mut reader = input_reader(args)?;
    reader.read_to_string(&mut buffer)?;

    let directions = Directions::from(&buffer);

    part_one(&directions);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
}
