// day
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{
    self,
    prelude::*,
    BufReader,
};
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq)]
enum Compliance {
    Compliant,
    NonCompliant,
}

#[derive(Debug)]
struct PasswordPolicy {
    min: usize,
    max: usize,
    letter: String,
}

impl PasswordPolicy {
    fn letter(&self) -> &str {
        &self.letter
    }

    fn min_max(&self) -> RangeInclusive<usize> {
        self.min ..= self.max
    }
}

impl From<&str> for PasswordPolicy {
    fn from(input: &str) -> Self {
        let policy_parts: Vec<&str> = input.split(' ').collect();
        let min_max: Vec<&str> = policy_parts[0].split('-').collect();
        let min: usize = min_max[0].parse().expect("Couldn't parse min number");
        let max: usize = min_max[1].parse().expect("Couldn't parse max number");

        Self {
            min: min,
            max: max,
            letter: policy_parts[1].into(),
        }
    }
}

#[derive(Debug)]
struct PasswordEntry {
    policy: PasswordPolicy,
    password: String,
}

impl From<&str> for PasswordEntry {
    fn from(input: &str) -> Self {
        let v: Vec<&str> = input.split(':').collect();
        let policy = v[0];
        let password = v[1];

        let password_policy: PasswordPolicy = policy.into();

        Self {
            policy: password_policy,
            password: password.trim().into(),
        }
    }
}

impl PasswordEntry {
    fn compliant(&self) -> Compliance {
        let letter = self.policy.letter();
        let range = self.policy.min_max();

        let count = self.password.chars()
            .filter(|c| c.to_string() == letter)
            .count();

        if range.contains(&count) {
            Compliance::Compliant
        }
        else {
            Compliance::NonCompliant
        }
    }
}

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

fn input_to_entries(input: &str) -> Vec<PasswordEntry> {
    input.lines()
        .map(|line| PasswordEntry::from(line))
        .collect()
}

fn part_one(input: &str) {
    let password_entries = input_to_entries(&input);

    let count = password_entries.iter()
        .map(|e| e.compliant())
        .filter(|c| c == &Compliance::Compliant)
        .count();

    println!("Found {} valid passwords", count);
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
