// day02
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{
    self,
    prelude::*,
    BufReader,
};

// CLI args
type Args = Vec<String>;

#[derive(Debug, Clone, PartialEq)]
enum Intcode {
    Add,
    Finished,
    Multiply,
    Unknown,
}

// Convert Intcodes back into strings
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

// Take strings and turn them into Intcodes
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

// Take i64 and turn them into Intcode
impl From<i64> for Intcode {
    fn from(s: i64) -> Self {
        match s {
            1  => Self::Add,
            2  => Self::Multiply,
            99 => Self::Finished,
            _  => Self::Unknown,
        }
    }
}

impl Intcode {
    fn instruction_length(&self) -> usize {
        match *self {
            Self::Add      => 4,
            Self::Finished => 1,
            Self::Multiply => 4,
            Self::Unknown  => 0,
        }
    }
}

// Size of an instruction on this architecture
const INSTRUCTION_SIZE: usize = 4;
const ADDR_VERB: usize = 1;
const ADDR_NOUN: usize = 2;
const ADDR_OUTPUT: usize = 0;

// Program memory definition
type Program = Vec<i64>;

// An Instruction is 3 usizes:
//   - input location A
//   - input location B
//   - output location
type Instruction = (usize, usize, usize);

#[derive(Debug, Default, Clone)]
struct Computer {
    counter: usize,
    program: Program,
    loaded:  bool,
}

// Implementation of a simple computer.
// A single computer can run multiple programs by calling `load` to load a new
// program and then `run`ning it.
impl Computer {
    fn new() -> Self {
        Default::default()
    }

    // Load a program from a tape
    fn load(&mut self, tape: &str) -> Result<(), Box<dyn Error>> {
        let mut program = Program::new();

        // Parse the data and load into program memory.
        for s in tape.split(",") {
            let num: i64 = s.trim().parse()?;
            program.push(num);
        }

        // A single computer could run multiple programs, ensure the counter is
        // 0 on each new load.
        self.counter = 0;

        self.program = program;
        self.loaded  = true;

        Ok(())
    }

    // Dump out the program memory
    #[allow(dead_code)]
    fn core_dump(&self) -> Option<&Program> {
        if self.loaded {
            Some(self.program.as_ref())
        }
        else {
            None
        }
    }

    // Peek at a memory location
    fn peek(&self, offset: usize) -> i64 {
        self.program[offset]
    }

    // Poke a value into program memory at a given offset
    fn poke(&mut self, offset: usize, value: i64) {
        self.program[offset] = value;
    }

    // Steps the program counter to the next set of instructions
    fn step(&mut self, step_size: usize) {
        self.counter = self.counter + step_size;
    }

    // Returns the opcode at the current program counter
    fn opcode(&self) -> Intcode {
        let offset = self.counter;
        let opcode = self.peek(offset);
        opcode.into()
    }

    // Returns the instruction at the current counter, excluding the opcode.
    fn instruction(&self) -> Instruction {
        let start = self.counter + 1; // Offset of 1 to exclude opcode
        let end   = self.counter + INSTRUCTION_SIZE;
        let range = start..end;

        let i = &self.program[range];

        (i[0] as usize, i[1] as usize, i[2] as usize)
    }

    // Perform addition on the values at in_loc_a and in_loc_b, storing the
    // result at out_loc.
    fn add(&mut self) {
        let (in_loc_a, in_loc_b, out_loc) = self.instruction();

        let sum = self.peek(in_loc_a) + self.peek(in_loc_b);
        self.poke(out_loc, sum);
    }

    // Perform multiplication on the values at in_loc_a and in_loc_b, storing
    // the result at out_loc.
    fn multiply(&mut self) {
        let (in_loc_a, in_loc_b, out_loc) = self.instruction();

        let product = self.peek(in_loc_a) * self.peek(in_loc_b);
        self.poke(out_loc, product);
    }

    // Execute the current instruction at the program counter location,
    // Returns a bool indicating if the program is finished
    fn execute(&mut self) -> bool {
        let opcode = self.opcode();
        let mut finished = false;

        match opcode {
            Intcode::Add => {
                let step_size = opcode.instruction_length();

                self.add();
                self.step(step_size);
            },
            Intcode::Finished => {
                finished = true;
            },
            Intcode::Multiply => {
                let step_size = opcode.instruction_length();

                self.multiply();
                self.step(step_size);
            },
            Intcode::Unknown => {
                eprintln!("Unknown opcode encountered: {}", opcode);
                finished = true;
            },
        }

        finished
    }

    // Run a program until completion
    fn run(&mut self) {
        loop {
            let finished = self.execute();

            if finished {
                break;
            }
        }
    }
}

// Create a buffered reader from either a file or stdin
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

// Part 1
// Simply execute the given program after restoring some state from the
// previous run.
fn part_one(data: &str) -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::new();
    computer.load(&data)?;

    // Restore the old state
    computer.poke(ADDR_VERB, 12);
    computer.poke(ADDR_NOUN, 2);

    computer.run();

    println!("State at pos {}: {}", ADDR_OUTPUT, computer.peek(ADDR_OUTPUT));

    Ok(())
}

// Part 2
// We're hunting for the needle, we just brute force it by iterating over all
// possible noun/verb conbinations.
fn part_two(data: &str) -> Result<(), Box<dyn Error>> {
    let mut computer = Computer::new();

    let needle     = 19690720;

    for noun in 0..=99 {
        for verb in 0..=99 {
            computer.load(&data)?;

            computer.poke(ADDR_VERB, verb);
            computer.poke(ADDR_NOUN, noun);

            computer.run();

            let output = computer.peek(ADDR_OUTPUT);

            if output == needle {
                println!("Found it! Verb: {}, Noun: {}", verb, noun);
                return Ok(())
            }
        }
    }

    Ok(())
}

// main
fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = env::args().collect();

    // Get the input
    let mut buffer = String::new();
    let mut reader = input_reader(args)?;
    reader.read_to_string(&mut buffer)?;

    part_one(&buffer)?;
    part_two(&buffer)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    // Take a core dump and stringify it
    fn core_to_string(core: &[i64]) -> String {
        core
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    #[test]
    fn test_intcode_from_str() {
        let ic: Intcode = "99".into();
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

    #[test]
    fn test_computer_execute() {
        let tests = vec![
            ("1,0,0,0,99",          "2,0,0,0,99"),
            ("2,3,0,3,99",          "2,3,0,6,99"),
            ("2,4,4,5,99,0",        "2,4,4,5,99,9801"),
            ("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99"),
        ];

        let mut computer = Computer::new();

        for (input, output) in tests {
            computer.load(input).unwrap();
            computer.run();

            let core = computer.core_dump().unwrap();
            let core = core_to_string(core);

            assert_eq!(core, output);
        }
    }
}
