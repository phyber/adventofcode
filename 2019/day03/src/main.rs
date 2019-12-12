// day03
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

// Moves that can be made
#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        // Incoming format is a single character direction followed by numbers.
        let (direction, count) = s.split_at(1);

        // Our input should always parse, panic if there's an issue.
        let count: usize = count.parse().unwrap();

        match direction {
            "U" => Self::Up(count),
            "D" => Self::Down(count),
            "L" => Self::Left(count),
            "R" => Self::Right(count),
            _   => unreachable!(),
        }
    }
}

// List of moves for a given wire.
type WirePath = Vec<Direction>;

// Wires that can exist in a position of the box.
#[derive(Debug, Clone)]
enum WireState {
    Empty,
    Blue,
    Red,
    Both,
}

// Current position in the grid
#[derive(Debug, Default, Clone)]
struct Position {
    x: usize,
    y: usize,
}

// 2D grid
type Grid = Vec<Vec<WireState>>;

#[derive(Debug, Default, Clone)]
struct Wirebox {
    position: Position,
    grid:     Grid,
    path:     WirePath,
}

impl Wirebox {
    fn new() -> Self {
        Default::default()
    }

    // Load the moves set from an incoming string
    fn load(&mut self, data: &str) {
        for d in data.split(",") {
            let direction: Direction = d.into();
            self.path.push(direction);
        }
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

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = env::args().collect();

    // Get the input
    let mut buffer = String::new();
    let mut reader = input_reader(args)?;
    reader.read_to_string(&mut buffer)?;

    let mut wirebox = Wirebox::new();
    wirebox.load(&buffer);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string_to_move() {
        let tests = vec![
            ("U1", Move::Up(1)),
            ("D2", Move::Down(2)),
            ("L3", Move::Left(3)),
            ("R4", Move::Right(4)),
        ];

        for (input, output) in tests {
            let direction: Move = input.into();

            assert_eq!(direction, output);
        }
    }
}
