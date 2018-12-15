use std::env;
use std::fs::File;
use std::io;

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
}
