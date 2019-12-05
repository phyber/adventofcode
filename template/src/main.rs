// day
use std::env;
use std::fs::File;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Either read from the given file or stdin
    let mut input: Box<io::Read> = if args.len() > 1 {
        let filename = &args[1];
        let fh = File::open(filename).unwrap();
        Box::new(fh)
    }
    else {
        let stdin = io::stdin();
        Box::new(stdin)
    };
}
