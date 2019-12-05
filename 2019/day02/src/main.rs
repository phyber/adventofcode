// day02
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct IntcodeError;

#[derive(Debug, Clone, PartialEq)]
enum Intcode {
    Add,
    Finished,
    Multiply,
    Unknown,
}

impl fmt::Display for Intcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::Add      => write!(f, "1"),
            Self::Finished => write!(f, "99"),
            Self::Multiply => write!(f, "2"),
            Self::Unknown  => write!(f, "UNKNOWN"),
        }
    }
}

impl FromStr for Intcode {
    type Err = IntcodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let intcode = match s {
            "1"  => Self::Add,
            "2"  => Self::Multiply,
            "99" => Self::Finished,
            _    => Self::Unknown,
        };

        Ok(intcode)
    }
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

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intcode_fromstr() {
        let ic = Intcode::from_str("99").unwrap();
        assert_eq!(ic, Intcode::Finished);
    }

    #[test]
    fn test_intcode_display() {
        let ic: Intcode = "99".parse().unwrap();
        assert_eq!(ic, Intcode::Finished);
    }
}
