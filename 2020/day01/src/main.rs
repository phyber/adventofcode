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

fn input_to_vec(input: &str) -> Vec<u64> {
    input.lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

// In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying
// them together produces 1721 * 299 = 514579, so the correct answer is 514579.
fn part_one(input: &[u64]) -> u64 {
    for (i, left) in input.iter().enumerate() {
        // Take entry and multiply it against every other number.
        for (j, right) in input.iter().enumerate() {
            // Don't check a number against itself
            if i == j {
                continue
            }

            let sum = left + right;

            if sum == 2020 {
                return left * right;
            }
        }
    }

    0
}

// Please, my code, she is dying.
fn part_two(input: &[u64]) -> u64 {
    for (i, left) in input.iter().enumerate() {
        // Take entry and multiply it against every other number.
        for (j, middle) in input.iter().enumerate() {
            // Don't check a number against itself
            if i == j {
                continue
            }

            for (k, right) in input.iter().enumerate() {
                if i == k || j == k {
                    continue
                }

                let sum = left + middle + right;

                if sum == 2020 {
                    return left * middle * right;
                }
            }
        }
    }

    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = env::args().collect();

    // Get the input
    let mut buffer = String::new();
    let mut reader = input_reader(args)?;
    reader.read_to_string(&mut buffer)?;

    let input = input_to_vec(&buffer);

    let ret = part_one(&input);

    println!("Part 1: {}", ret);

    let ret = part_two(&input);

    println!("Part 2: {}", ret);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
}
