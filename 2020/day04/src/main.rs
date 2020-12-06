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
enum HeightType {
    Centimetres(u64),
    Inches(u64),
}

#[derive(Debug)]
enum Detail {
    BirthYear(u64),
    CountryId(String),
    ExpirationYear(u64),
    EyeColour(String),
    HairColour(String),
    Height(HeightType),
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
            "hgt" => {
                // Collect the height, ignoring the suffix.
                let height: String = value.chars()
                    .filter(|c| c.is_numeric())
                    .collect();

                // Parse the height string to a u64
                let height: u64 = height.parse().unwrap();

                // Check the suffix to see which height type we have
                let height_type = if value.ends_with("cm") {
                    HeightType::Centimetres(height)
                }
                else {
                    HeightType::Inches(height)
                };

                Self::Height(height_type)
            },
            "iyr" => Self::IssueYear(value.parse().unwrap()),
            "pid" => Self::PassportId(value.into()),
            _     => panic!("Unknown password field"),
        }
    }
}

#[derive(Debug, Default)]
struct BirthYear(Option<u64>);

impl BirthYear {
    fn validate_one(&self) -> Validation {
        match self.0 {
            None    => Validation::Invalid,
            Some(_) => Validation::Valid,
        }
    }

    fn validate_two(&self) -> Validation {
        match self.0 {
            None      => Validation::Invalid,
            Some(byr) => {
                match byr {
                    1920 ..= 2002 => Validation::Valid,
                    _             => Validation::Invalid,
                }
            }
        }
    }
}

#[derive(Debug, Default)]
struct CountryId(Option<String>);

impl CountryId {
    fn validate_one(&self) -> Validation {
        // CountryId is optional so always valid
        Validation::Valid
    }

    fn validate_two(&self) -> Validation {
        // CountryId is optional so always valid
        Validation::Valid
    }
}

#[derive(Debug, Default)]
struct ExpirationYear(Option<u64>);

impl ExpirationYear {
    fn validate_one(&self) -> Validation {
        match self.0 {
            None    => Validation::Invalid,
            Some(_) => Validation::Valid,
        }
    }

    fn validate_two(&self) -> Validation {
        match self.0 {
            None      => Validation::Invalid,
            Some(eyr) => {
                match eyr {
                    2020 ..= 2030 => Validation::Valid,
                    _             => Validation::Invalid,
                }
            }
        }
    }
}

#[derive(Debug, Default)]
struct EyeColour(Option<String>);

impl EyeColour {
    fn validate_one(&self) -> Validation {
        match self.0 {
            None    => Validation::Invalid,
            Some(_) => Validation::Valid,
        }
    }

    fn validate_two(&self) -> Validation {
        let valid_colours = vec![
            "amb",
            "blu",
            "brn",
            "gry",
            "grn",
            "hzl",
            "oth",
        ];

        match &self.0 {
            None      => Validation::Invalid,
            Some(ecl) => {
                if !valid_colours.contains(&ecl.as_str()) {
                    return Validation::Invalid;
                }

                Validation::Valid
            }
        }
    }
}

#[derive(Debug, Default)]
struct HairColour(Option<String>);

impl HairColour {
    fn validate_one(&self) -> Validation {
        match self.0 {
            None    => Validation::Invalid,
            Some(_) => Validation::Valid,
        }
    }

    fn validate_two(&self) -> Validation {
        match &self.0 {
            None      => Validation::Invalid,
            Some(hcl) => {
                // A # followed by 6 characters
                if !hcl.starts_with("#") || hcl.len() != 7 {
                    return Validation::Invalid;
                }

                // Ensure that hcl is only composed of hex digits.
                // Skip the first #
                if hcl.chars().skip(1).any(|c| !c.is_digit(16)) {
                    return Validation::Invalid;
                }

                Validation::Valid
            }
        }
    }
}

#[derive(Debug, Default)]
struct Height(Option<HeightType>);

impl Height {
    fn validate_one(&self) -> Validation {
        match self.0 {
            None    => Validation::Invalid,
            Some(_) => Validation::Valid,
        }
    }

    fn validate_two(&self) -> Validation {
        match &self.0 {
            None              => Validation::Invalid,
            Some(height_type) => {
                match height_type {
                    HeightType::Centimetres(cm) => {
                        match cm {
                            150 ..= 193 => Validation::Valid,
                            _           => Validation::Invalid,
                        }
                    },
                    HeightType::Inches(i) => {
                        match i {
                            59 ..= 76 => Validation::Valid,
                            _         => Validation::Invalid,
                        }
                    },
                }
            }
        }
    }
}

#[derive(Debug, Default)]
struct IssueYear(Option<u64>);

impl IssueYear {
    fn validate_one(&self) -> Validation {
        match self.0 {
            None    => Validation::Invalid,
            Some(_) => Validation::Valid,
        }
    }

    fn validate_two(&self) -> Validation {
        match self.0 {
            None      => Validation::Invalid,
            Some(iyr) => {
                match iyr {
                    2010 ..= 2020 => Validation::Valid,
                    _             => Validation::Invalid,
                }
            }
        }
    }
}

#[derive(Debug, Default)]
struct PassportId(Option<String>);

impl PassportId {
    fn validate_one(&self) -> Validation {
        match self.0 {
            None    => Validation::Invalid,
            Some(_) => Validation::Valid,
        }
    }

    fn validate_two(&self) -> Validation {
        match &self.0 {
            None      => Validation::Invalid,
            Some(pid) => {
                if pid.chars().any(|c| !c.is_digit(10)) {
                    return Validation::Invalid;
                }

                Validation::Valid
            }
        }
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
    fn validate_one(&self) -> Validation {
        // The required passport fields
        let required = vec![
            self.birth_year.validate_one(),
            //self.country_id.validate_one(),
            self.expiration_year.validate_one(),
            self.eye_colour.validate_one(),
            self.hair_colour.validate_one(),
            self.height.validate_one(),
            self.issue_year.validate_one(),
            self.passport_id.validate_one(),
        ];

        // Fail if any required field didn't pass validation
        if required.iter().any(|f| *f == Validation::Invalid) {
            return Validation::Invalid
        }

        // Finally
        Validation::Valid
    }

    fn validate_two(&self) -> Validation {
        // The required passport fields
        let required = vec![
            self.birth_year.validate_two(),
            //self.country_id.validate_two(),
            self.expiration_year.validate_two(),
            self.eye_colour.validate_two(),
            self.hair_colour.validate_two(),
            self.height.validate_two(),
            self.issue_year.validate_two(),
            self.passport_id.validate_two(),
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

    passports.push(passport);
    passports
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = env::args().collect();

    // Get the input
    let mut buffer = String::new();
    let mut reader = input_reader(args)?;
    reader.read_to_string(&mut buffer)?;

    let passports = input_to_passports(&buffer);

    let day_one_valid_total = passports.iter()
        .map(|p| p.validate_one())
        .filter(|v| *v == Validation::Valid)
        .count();

    println!("Day 1 valid: {}", day_one_valid_total);

    let day_two_valid_total = passports.iter()
        .map(|p| p.validate_two())
        .filter(|v| *v == Validation::Valid)
        .count();

    println!("Day 2 valid: {}", day_two_valid_total);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_passports_validate_one() {
        // Taken from example on day 4
        let passports = vec![
            Passport {
                eye_colour: EyeColour(Some("gry".into())),
                passport_id: PassportId(Some("860033327".into())),
                expiration_year: ExpirationYear(Some(2020)),
                hair_colour: HairColour(Some("#fffffd".into())),
                birth_year: BirthYear(Some(1937)),
                issue_year: IssueYear(Some(2017)),
                country_id: CountryId(Some("147".into())),
                height: Height(Some("183cm".into())),
            },
            Passport {
                issue_year: IssueYear(Some(2013)),
                eye_colour: EyeColour(Some("amb".into())),
                country_id: CountryId(Some("350".into())),
                expiration_year: ExpirationYear(Some(2023)),
                passport_id: PassportId(Some("028048884".into())),
                hair_colour: HairColour(Some("#cfa07d".into())),
                birth_year: BirthYear(Some(1929)),
                ..Default::default()
            },
            Passport {
                hair_colour: HairColour(Some("#ae17e1".into())),
                issue_year: IssueYear(Some(2013)),
                expiration_year: ExpirationYear(Some(2024)),
                eye_colour: EyeColour(Some("brn".into())),
                passport_id: PassportId(Some("760753108".into())),
                birth_year: BirthYear(Some(1931)),
                height: Height(Some("179cm".into())),
                ..Default::default()
            },
            Passport {
                hair_colour: HairColour(Some("#cfa07d".into())),
                expiration_year: ExpirationYear(Some(2025)),
                passport_id: PassportId(Some("166559648".into())),
                issue_year: IssueYear(Some(2011)),
                eye_colour: EyeColour(Some("brn".into())),
                height: Height(Some("59in".into())),
                ..Default::default()
            },
        ];

        let valid_count = passports.iter()
            .map(|p| p.validate_one())
            .filter(|v| *v == Validation::Valid)
            .count();

        for (i, passport) in passports.iter().enumerate() {
            println!("{}: {:?}", i, passport.validate_one())
        }

        assert_eq!(valid_count, 2);
    }
}
