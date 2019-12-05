// day
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{
    self,
    prelude::*,
    BufReader,
};

fn fuel_required(mass: u64) -> u64 {
    (mass as f64 / 3.0).floor() as u64 - 2
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

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
    let mut total_fuel_required = 0;

    for line in reader.lines() {
        let mass: u64 = line?.parse()?;
        let fuel = fuel_required(mass);

        total_fuel_required = total_fuel_required + fuel;
    }

    println!("Fuel required: {}", total_fuel_required);

    Ok(())
}
