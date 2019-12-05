// day02
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;

type Program = Vec<i64>;

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

impl From<&str> for Intcode {
    fn from(s: &str) -> Self {
        match s {
            "1"  => Self::Add,
            "2"  => Self::Multiply,
            "99" => Self::Finished,
            _    => Self::Unknown,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // Either read from the given file or stdin
    let mut input: Box<dyn io::Read> = if args.len() > 1 {
        let filename = &args[1];
        let fh = File::open(filename).unwrap();
        Box::new(fh)
    }
    else {
        let stdin = io::stdin();
        Box::new(stdin)
    };

    let mut buffer = String::new();
    input.read_to_string(&mut buffer)?;

    // Get a new program space
    let mut program = Program::new();

    // Parse the intcode string into a program vec
    for s in buffer.split(",") {
        let num: i64 = s.trim().parse()?;
        program.push(num);
    }

    println!("{:?}", program);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intcode_from_str() {
        let ic: Intcode = "99".into(); //Intcode::from_str("99").unwrap();
        assert_eq!(ic, Intcode::Finished);
    }

    #[test]
    fn test_intcode_into_string() {
        let ic = Intcode::Finished;
        let s = ic.to_string();

        assert_eq!("99", &s);
    }

    #[test]
    fn test_intcode_display() {
        let s = format!("{}", Intcode::Finished);
        assert_eq!("99", &s);
    }
}
