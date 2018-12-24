use std::env;
use std::fs::File;
use std::io;
use regex::{
    Captures,
    Regex,
};

const CLAIM_PATTERN: &'static str = r"(?x)  # Ignore whitespace, allow comments
    ^\#(?P<id>\d+)                          # Claim ID: id
    \s@\s                                   # @
    (?P<x>\d+),(?P<y>\d+):\s                # Anchor points: x,y
    (?P<width>\d+)x(?P<height>\d+)$         # Rectangle size: WxH
    ";

#[derive(Debug)]
struct Rectangle {
    width: i64,
    height: i64,
}

#[derive(Debug)]
struct Anchor {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Claim {
    id: i64,
    anchor: Anchor,
    rectangle: Rectangle,
}

impl Claim {
    // Checks if any rectangle bounds overlap
    fn overlap(&self, c: &Claim) -> bool {
        false
    }

    // Returns the size of the overlapping area
    // NYI
}

fn capture_to_i64(c: &Captures, name: &str) -> i64 {
    c.name(name).unwrap().as_str().parse().unwrap()
}

fn process_input(mut inp: impl io::Read) -> Vec<Claim> {
    // Read the input into a buffer
    let mut buffer = String::new();
    inp.read_to_string(&mut buffer).unwrap();

    // Regex
    let re = Regex::new(CLAIM_PATTERN).unwrap();

    // Storage for processed claims.
    let mut claims: Vec<Claim> = Vec::new();

    for line in buffer.lines() {
        let captures = re.captures(line).unwrap();

        let r = Rectangle{
            width: capture_to_i64(&captures, "width"),
            height: capture_to_i64(&captures, "height"),
        };

        let a = Anchor{
            x: capture_to_i64(&captures, "x"),
            y: capture_to_i64(&captures, "y"),
        };

        let c = Claim{
            id: capture_to_i64(&captures, "id"),
            anchor: a,
            rectangle: r,
        };

        claims.push(c);
    }

    claims
}

fn main() {
    // Collect command line args
    let args: Vec<String> = env::args().collect();

    // If we got an arg, use it as the input, otherwise use stdin
    let input: Box<io::Read> = if args.len() > 1 {
        let filename = &args[1];
        let fh = File::open(filename).unwrap();
        Box::new(fh)
    }
    else {
        let stdin = io::stdin();
        Box::new(stdin)
    };

    let claims = process_input(input);

    println!("{:?}", claims);
}
