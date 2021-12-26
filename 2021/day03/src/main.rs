// day
use std::collections::HashMap;
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

struct Counts(HashMap<usize, usize>);

impl Counts {
    fn new() -> Self {
        let hashmap = HashMap::new();

        Counts(hashmap)
    }

    fn add(&mut self, s: &str) {
        //s.chars()
        //    .enumerate()
        //    .for_each(|(index, digit)| {
        //        match digit {
        //            "1" => self.0.
        // Get a count of the number of 1s in each index position.
        for (index, digit) in s.chars().enumerate() {
            if digit == '1' {
                let counter = self.0.entry(index).or_insert(0);
                *counter += 1;
            }
        }
    }

    // Based on the total number of lines, we get our binary back out.
    fn binary(&self, total: usize) -> String {
        let map_size = self.0.len();
        let mut output = Vec::with_capacity(map_size);

        // Fill with zeros to ensure we have indexes available for later.
        for _i in 0..output.capacity() {
            output.push("0");
        }

        for (index, value) in &self.0 {
            let ones = value;
            let zeros = total - ones;

            let insert = if *ones > zeros {
                "1"
            }
            else {
                "0"
            };

            output[*index] = insert;
        }

        output.join("")
    }

    // Count of the 1s in binary
    fn gamma(&self, total: usize) -> String {
        self.binary(total)
    }

    fn epsilon(&self, total: usize) -> String {
        let gamma = self.gamma(total);
        let int = usize::from_str_radix(&gamma, 2).expect("valid binary");
        let inverted = !int;
        let binary = format!("{:b}", inverted);

        binary[binary.len() - 12..].to_string()
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

fn part_one(input: &str) {
    let mut counts = Counts::new();
    let mut lines = 0;

    for line in input.lines() {
        counts.add(&line);
        lines += 1;
    }

    let gamma = counts.gamma(lines);
    let epsilon = counts.epsilon(lines);

    println!("Gamma: {}", gamma);
    println!("Epsilon: {}", epsilon);

    let gamma_int = usize::from_str_radix(&gamma, 2).expect("valid gamma");
    let epsilon_int = usize::from_str_radix(&epsilon, 2).expect("valid epsilon");

    println!("Part 1: {}", gamma_int * epsilon_int);
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = env::args().collect();

    // Get the input
    let mut buffer = String::new();
    let mut reader = input_reader(args)?;
    reader.read_to_string(&mut buffer)?;

    part_one(&buffer);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
}
