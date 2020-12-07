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

fn input_to_part_one_questions(input: &str) -> Vec<usize> {
    let mut yes: Vec<usize> = Vec::new();
    let mut tmp: Vec<char> = Vec::new();

    for line in input.lines() {
        if line.len() > 0 {
            for c in line.chars() {
                tmp.push(c);
            }
        }
        else {
            tmp.sort();
            tmp.dedup();
            yes.push(tmp.len());
            tmp = Vec::new();
        }
    }

    tmp.sort();
    tmp.dedup();
    yes.push(tmp.len());

    yes
}

fn part_one(q: &[usize]) {
    let total: usize = q.iter().sum();

    println!("Part 1: Yes = {}", total);
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = env::args().collect();

    // Get the input
    let mut buffer = String::new();
    let mut reader = input_reader(args)?;
    reader.read_to_string(&mut buffer)?;

    let questions = input_to_part_one_questions(&buffer);

    part_one(&questions);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
}
