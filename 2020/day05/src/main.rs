// day
use std::cmp::Ordering;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{
    self,
    prelude::*,
    BufReader,
};

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct SeatId(u64);

impl fmt::Display for SeatId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Ord for SeatId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl From<&str> for SeatId {
    fn from(input: &str) -> Self {
        let mut rows: Vec<u64> = (0 ..= 127).collect();
        let mut cols: Vec<u64> = (0 ..= 7).collect();

        for c in input.chars() {
            match c {
                'F' => {
                    let len = rows.len();
                    let split = len / 2;

                    // Split "seats", we don't need this half since we want
                    // the front half that's left in "seats".
                    let _ = rows.split_off(split);
                },
                'B' => {
                    let len = rows.len();
                    let split = len / 2;

                    // With the back half, we want the new split.
                    let new = rows.split_off(split);
                    rows = new;
                },
                'L' => {
                    let len = cols.len();
                    let split = len / 2;

                    let _ = cols.split_off(split);
                },
                'R' => {
                    let len = cols.len();
                    let split = len / 2;

                    let new = cols.split_off(split);
                    cols = new;
                },
                _ => panic!("Unknown seat assignment code"),
            }
        }

        // Should only be a single row and column left now.
        let row = rows[0];
        let col = cols[0];
        let seat_id = (row * 8) + col;

        Self(seat_id)
    }
}

// CLI arguments
type Args = Vec<String>;

fn part_one(input: &str) {
    let seat_ids: Vec<SeatId> = input.lines()
        .map(|line| SeatId::from(line))
        .collect();

    let max = seat_ids.iter()
        .max()
        .unwrap();

    println!("Part 1: {}", max);
}

fn part_two(input: &str) {
    let seat_ids: Vec<SeatId> = input.lines()
        .map(|line| SeatId::from(line))
        .collect();

    let min = seat_ids.iter()
        .min()
        .unwrap();

    let max = seat_ids.iter()
        .max()
        .unwrap();

    // The sum if every seat was full
    let clean_sum: u64 = (min.0 ..= max.0).sum();

    // The sum with a missing seat
    let real_sum: u64 = seat_ids.iter()
        .fold(0, |acc, x| acc + x.0);

    // Find the difference between the two, this should be our seat number.
    let seat_id = clean_sum - real_sum;

    println!("Part 2: Min {}, Max {}", min, max);
    println!("Part 2: SeatId {}", seat_id);
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

    part_one(&buffer);
    part_two(&buffer);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_seatid_from() {
        let input = "FBFBBFFRLR";
        let expected = SeatId(357);

        let seat_id: SeatId = input.into();

        assert_eq!(seat_id, expected);
    }
}
