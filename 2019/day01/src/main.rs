// day
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{
    self,
    prelude::*,
    BufReader,
};

fn fuel_for_fuel(mass: i64) -> i64 {
    let mut additional_fuel = 0;
    let mut next_mass = mass;

    while next_mass > 0 {
        let required = fuel_required(next_mass);

        if required < 0 {
            break;
        }

        additional_fuel = additional_fuel + required;
        next_mass = required as i64;
    }

    additional_fuel
}

fn fuel_required(mass: i64) -> i64 {
    let fuel = (mass as f64 / 3.0).floor() as i64 - 2;
    fuel
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
    let mut mass_fuel_required = 0;
    let mut fuel_fuel_required = 0;
    let mut line_count = 1;

    for line in reader.lines() {
        let mass: i64 = line?.parse()?;
        let fuel = fuel_required(mass);
        let fuel_fuel = fuel_for_fuel(fuel as i64);

        println!("{line}: {mass} -> {fuel} -> {fuel_fuel}",
            line=line_count,
            mass=mass,
            fuel=fuel,
            fuel_fuel=fuel_fuel,
        );

        line_count = line_count + 1;

        mass_fuel_required = mass_fuel_required + fuel;
        fuel_fuel_required = fuel_fuel_required + fuel_fuel;
    }

    println!("Fuel required for base mass: {}", mass_fuel_required);
    println!("Fuel required for fuel: {}", fuel_fuel_required);
    println!("Fuel required total: {}", mass_fuel_required + fuel_fuel_required);

    Ok(())
}
