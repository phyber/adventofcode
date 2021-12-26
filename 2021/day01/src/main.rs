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

fn str_to_nums(input: &str) -> Vec<usize> {
    input.lines()
        .map(|s| s.parse().expect("valid integer"))
        .collect()
}

fn part_one(input: &[usize]) {
    let mut increases = 0;

    // Get the starting point for depth
    let mut depth = input[0];

    // Skip the first element, we already took it above
    for num in input.iter().skip(1) {
        if *num > depth {
            increases += 1;
        }

        depth = *num;
    }

    println!("Part 1 increases: {}", increases);
}

fn part_two(input: &[usize]) {
    let mut increases = 0;

    // Get the first set of values and sum them for future comparison
    let mut depth = input.get(0..3)
        .unwrap()
        .iter()
        .sum();

    // Index for getting our numbers
    let mut n = 1;

    while let Some(nums) = input.get(n .. n + 3) {
        let sum: usize = nums.iter().sum();

        if sum > depth {
            increases += 1;
        }

        depth = sum;
        n += 1;
    }

    println!("Part 2 increases: {}", increases);
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = env::args().collect();

    // Get the input
    let mut buffer = String::new();
    let mut reader = input_reader(args)?;
    reader.read_to_string(&mut buffer)?;

    let nums = str_to_nums(&buffer);

    part_one(&nums);
    part_two(&nums);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
}
