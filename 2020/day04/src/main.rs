// day
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{
    self,
    prelude::*,
    BufReader,
};

#[derive(Debug)]
enum Detail {
    BirthYear(u64),
    CountryId(String),
    ExpirationYear(u64),
    EyeColour(String),
    HairColour(String),
    Height(String),
    IssueYear(u64),
    PassportId(String),
}

impl From<&str> for Detail {
    fn from(input: &str) -> Self {
        let v: Vec<&str> = input.split(':').collect();
        let field = v[0];
        let value = v[1];

        match field {
            "byr" => Self::BirthYear(value.parse().unwrap()),
            "cid" => Self::CountryId(value.into()),
            "eyr" => Self::ExpirationYear(value.parse().unwrap()),
            "ecl" => Self::EyeColour(value.into()),
            "hcl" => Self::HairColour(value.into()),
            "hgt" => Self::Height(value.into()),
            "iyr" => Self::IssueYear(value.parse().unwrap()),
            "pid" => Self::PassportId(value.into()),
            _     => panic!("Unknown password field"),
        }
    }
}

#[derive(Debug, Default)]
struct BirthYear(Option<u64>);

impl BirthYear {
    fn validate(&self) -> Validation {
        Validation::Valid
    }
}

#[derive(Debug, Default)]
struct CountryId(Option<String>);

impl CountryId {
    fn validate(&self) -> Validation {
        Validation::Valid
    }
}

#[derive(Debug, Default)]
struct ExpirationYear(Option<u64>);

impl ExpirationYear {
    fn validate(&self) -> Validation {
        Validation::Valid
    }
}

#[derive(Debug, Default)]
struct EyeColour(Option<String>);

impl EyeColour {
    fn validate(&self) -> Validation {
        Validation::Valid
    }
}

#[derive(Debug, Default)]
struct HairColour(Option<String>);

impl HairColour {
    fn validate(&self) -> Validation {
        Validation::Valid
    }
}

#[derive(Debug, Default)]
struct Height(Option<String>);

impl Height {
    fn validate(&self) -> Validation {
        Validation::Valid
    }
}

#[derive(Debug, Default)]
struct IssueYear(Option<u64>);

impl IssueYear {
    fn validate(&self) -> Validation {
        Validation::Valid
    }
}

#[derive(Debug, Default)]
struct PassportId(Option<String>);

impl PassportId {
    fn validate(&self) -> Validation {
        Validation::Valid
    }
}

#[derive(Debug, PartialEq)]
enum Validation {
    Invalid,
    Valid,
}

#[derive(Debug, Default)]
struct Passport {
    birth_year: BirthYear,
    country_id: CountryId,
    expiration_year: ExpirationYear,
    eye_colour: EyeColour,
    hair_colour: HairColour,
    height: Height,
    issue_year: IssueYear,
    passport_id: PassportId,
}

impl Passport {
    fn validate(&self) -> Validation {
        // The required passport fields
        let required = vec![
            self.birth_year.validate(),
            // country_id is optional
            self.expiration_year.validate(),
            self.eye_colour.validate(),
            self.hair_colour.validate(),
            self.height.validate(),
            self.issue_year.validate(),
            self.passport_id.validate(),
        ];

        // Fail if any required field didn't pass validation
        if required.iter().any(|f| *f == Validation::Invalid) {
            return Validation::Invalid
        }

        // Finally
        Validation::Valid
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

fn input_to_passports(input: &str) -> Vec<Passport> {
    // Accumulates passports as we parse them
    let mut passports: Vec<Passport> = Vec::new();

    // This is the active passport. This will eventually end up in passports
    // and be reset.
    let mut passport: Passport = Default::default();

    for line in input.lines() {
        if line.len() > 0 {
            let details: Vec<&str> = line.split(' ').collect();

            for detail in details.into_iter() {
                let detail: Detail = detail.into();

                match detail {
                    Detail::BirthYear(byr) => {
                        passport.birth_year = BirthYear(Some(byr))
                    },
                    Detail::CountryId(cid) => {
                        passport.country_id = CountryId(Some(cid))
                    },
                    Detail::ExpirationYear(eyr) => {
                        passport.expiration_year = ExpirationYear(Some(eyr))
                    },
                    Detail::EyeColour(ecl) => {
                        passport.eye_colour = EyeColour(Some(ecl))
                    },
                    Detail::HairColour(hcl) => {
                        passport.hair_colour = HairColour(Some(hcl))
                    },
                    Detail::Height(hgt) => {
                        passport.height = Height(Some(hgt))
                    },
                    Detail::IssueYear(iyr) => {
                        passport.issue_year = IssueYear(Some(iyr))
                    },
                    Detail::PassportId(pid) => {
                        passport.passport_id = PassportId(Some(pid))
                    },
                }
            }
        }
        else {
            passports.push(passport);
            passport = Default::default();
        }
    }

    passports
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = env::args().collect();

    // Get the input
    let mut buffer = String::new();
    let mut reader = input_reader(args)?;
    reader.read_to_string(&mut buffer)?;

    let passports = input_to_passports(&buffer);

    println!("{:#?}", passports);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
}
