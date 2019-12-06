// day02
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;

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

// Size of an instruction on this architecture
const INSTRUCTION_SIZE: usize = 4;

// Program memory definition
type Program = Vec<i64>;

type Instruction = (usize, usize, usize);

#[derive(Debug, Default, Clone)]
struct Computer {
    counter:  usize,
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

        for s in tape.split(",") {
            let num: i64 = s.trim().parse()?;
            program.push(num);
        }

        self.counter = 0;
        self.program = program;
        self.loaded  = true;

        Ok(())
    }

    // Dump out the program memory
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
    fn step(&mut self) {
        self.counter = self.counter + INSTRUCTION_SIZE;
    }

    // Returns the opcode at the current program counter
    fn opcode(&self) -> i64 {
        let offset = self.counter;
        self.peek(offset)
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

        match opcode.into() {
            Intcode::Add => {
                self.add();
                self.step();
            },
            Intcode::Finished => {
                finished = true;
            },
            Intcode::Multiply => {
                self.multiply();
                self.step();
            }
            Intcode::Unknown => {
                eprintln!("Unknown opcode encountered: {}", opcode);
            }
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

    let mut computer = Computer::new();
    computer.load(&buffer)?;

    println!("{:?}", computer.core_dump());

    // Restore the old state
    computer.poke(1, 12);
    computer.poke(2, 2);

    computer.run();

    println!("State at pos 0: {}", computer.peek(0));

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

    #[test]
    fn test_computer_execute() {
        let tests = vec![
            ("1,0,0,0,99", "2,0,0,0,99"),
            ("2,3,0,3,99", "2,3,0,6,99"),
            ("2,4,4,5,99,0", "2,4,4,5,99,9801"),
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
