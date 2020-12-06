// day
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{
    self,
    prelude::*,
    BufReader,
};

#[derive(Debug, PartialEq)]
enum Tile {
    Ground,
    Tree,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Self::Ground => ".",
            Self::Tree   => "#",
        };

        write!(f, "{}", output)
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Ground,
            '#' => Self::Tree,
            _   => panic!("Unknown tile"),
        }
    }
}

#[derive(Debug, Default)]
struct Worldstate(Vec<Vec<Tile>>);

impl Worldstate {
    fn rows(&self) -> usize {
        self.0.len()
    }

    // Could panic if used before Worldstate is initialized
    fn cols(&self) -> usize {
        self.0[0].len()
    }

    fn row(&self, row: usize) -> &[Tile] {
        &self.0[row]
    }

    // Could panic if used before Worldstate is initialized
    fn tile(&self, row: usize, col: usize) -> &Tile {
        let row = self.row(row);
        let tile = &row[col];

        tile
    }
}

impl From<String> for Worldstate {
    fn from(input: String) -> Self {
        let mut state: Self = Default::default();

        for line in input.lines() {
            let row = line.chars()
                .map(|c| Tile::from(c))
                .collect();

            state.0.push(row);
        }

        state
    }
}

#[derive(Debug)]
struct Worldrow(Vec<Tile>);

// CLI arguments
type Args = Vec<String>;

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

fn part_one(state: &Worldstate) {
    let step_rows    = 1;
    let step_columns = 3;

    let total_rows = state.rows();
    let total_cols = state.cols();

    let mut row = 0;
    let mut col = 0;
    let mut trees = 0;

    loop {
        // Go down a number of rows
        row += step_rows;

        // Bounds check
        if row >= total_rows {
            break
        }

        //println!("Row {}: {:?}", row, state.row(row));

        // Go right a number of columns
        col += step_columns;

        // Wrapping
        col %= total_cols;

        // Get the tile
        let tile = state.tile(row, col);

        // Is it a tree?
        if tile == &Tile::Tree {
            println!("Tree at: {}x{}", row, col);

            trees += 1;
        }
    }

    println!("Part 1: Encountered {} trees", trees);
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = env::args().collect();

    // Get the input
    let mut buffer = String::new();
    let mut reader = input_reader(args)?;
    reader.read_to_string(&mut buffer)?;

    let state: Worldstate = buffer.into();
    //println!("{:?}", state);

    part_one(&state);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
}
