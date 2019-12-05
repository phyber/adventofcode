use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io;

type FrequencyCount = HashMap<i64, i64>;

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

    // Frequency tracking
    let mut freq: i64 = 0;

    // Map trackin which frequencies we've seen
    let mut seen = FrequencyCount::new();

    // Keep track of calibration status
    let mut calibrated = false;

    // Tracking of the first loop to output day01a value
    let mut first_loop = true;

    // Loop over lines in buffer until we're calibrated
    while !calibrated {
        for line in buffer.lines() {
            // Get a mutable version of the line so we can split_off
            let mut line = line.to_owned();

            // Split number out of line, leaving sign
            let num = line.split_off(1);

            // Get the sign
            let sign = line;

            // Convert number to an i64
            let value: i64 = num.parse().unwrap();

            // Perform some math on the frequency depending on the sign
            freq = match sign.as_ref() {
                "+" => freq + value,
                "-" => freq - value,
                _ => { unreachable!() },
            };

            // Maintain a hash of what we've seen
            *seen.entry(freq).or_insert(0) += 1;

            // Check if we've seen this frequency twice or not.
            if Some(&2) == seen.get(&freq) {
                // We have it, break out of the loop.
                calibrated = true;
                break;
            }
        }

        // If this is the first loop, output the frequency answer for day01a.
        if first_loop {
            println!("Freq: {}", freq);
            first_loop = false;
        }
    }

    // Output calibration value
    println!("Calibrated: {}", freq);
}
