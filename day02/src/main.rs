use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;

type LetterCounts = HashMap<char, i64>;
type Multipliers = HashMap<i64, i64>;

// Compare two given strings returning the numbers of characters that differ.
fn compare(a: &str, b: &str) -> usize {
    let mut diff = 0;

    for (i, c) in a.char_indices() {
        if c != b.chars().nth(i).unwrap() {
            diff += 1;
        }
    }

    diff
}

fn main() {
    // Collect command line args
    let args: Vec<String> = env::args().collect();

    // If we got an arg, use it as the input, otherwise use stdin
    let mut input: Box<io::Read> = if args.len() > 1 {
        let filename = &args[1];
        let fh = File::open(filename).unwrap();
        Box::new(fh)
    }
    else {
        let stdin = io::stdin();
        Box::new(stdin)
    };

    // Read the input into a buffer
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    let mut totals = Multipliers::new();

    for line in buffer.lines() {
        let mut counts = LetterCounts::new();

        let mut two = false;
        let mut three = false;

        // Count characters in the line
        for ch in line.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }

        for val in counts.values() {
            match val {
                2 => two = true,
                3 => three = true,
                _ => {},
            }
        }

        if two {
            *totals.entry(2).or_insert(0) += 1;
        }

        if three {
            *totals.entry(3).or_insert(0) += 1;
        }

        // Work out diffs
        // line from above is A, each line in lines is B.
        let lines = buffer.clone();
        for b in lines.lines() {
            let diff = compare(line, b);

            if diff == 1 {
                println!("COMP: {} -> {}", line, b);
            }
        }
    }

    // We now want to multiply the 2s by the 3s.
    let twos = totals.get(&2).unwrap();
    let threes = totals.get(&3).unwrap();

    let checksum = twos * threes;
    println!("Checksum: {}", checksum);
}
