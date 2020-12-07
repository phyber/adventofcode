// day
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{
    self,
    prelude::*,
    BufReader,
};

// CLI arguments
type Args = Vec<String>;

// Vec of questions a person answered yes to
#[derive(Clone, Debug, Default)]
struct Person(Vec<char>);

impl Person {
    fn push(&mut self, c: char) {
        self.0.push(c);
    }
}

// Group of people that answered questions
#[derive(Clone, Debug, Default)]
struct Group(Vec<Person>);

impl Group {
    fn push(&mut self, person: &Person) {
        self.0.push(person.clone());
    }

    // Return a count of how many questions each person in a group had in
    // common
    fn same_answers(&self) -> usize {
        let mut counter: HashMap<&char, usize> = HashMap::new();

        for person in &self.0 {
            for c in &person.0 {
                *counter.entry(c).or_insert(0) += 1;
            }
        }

        let group_size = self.0.len();
        let mut same_answers = 0;

        for (_, num) in counter.iter() {
            if *num == group_size {
                same_answers += 1;
            }
        }

        same_answers
    }
}

#[derive(Debug, Default)]
struct Groups(Vec<Group>);

impl Groups {
    fn push(&mut self, group: &Group) {
        self.0.push(group.clone());
    }

    fn same_answers(&self) -> usize {
        self.0.iter()
            .map(|g| g.same_answers())
            .sum()
    }
}

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

fn input_to_part_one_questions(input: &str) -> Vec<usize> {
    let mut yes: Vec<usize> = Vec::new();
    let mut tmp: Vec<char> = Vec::new();

    for line in input.lines() {
        if line.len() > 0 {
            for c in line.chars() {
                tmp.push(c);
            }
        }
        else {
            tmp.sort();
            tmp.dedup();
            yes.push(tmp.len());
            tmp = Vec::new();
        }
    }

    tmp.sort();
    tmp.dedup();
    yes.push(tmp.len());

    yes
}

fn input_to_part_two_questions(input: &str) -> Groups {
    let mut groups: Groups = Default::default();
    let mut group: Group = Default::default();

    for line in input.lines() {
        if line.len() > 0 {
            // Each line is a new person
            let mut person: Person = Default::default();

            for c in line.chars() {
                person.push(c)
            }

            group.push(&person);
        }
        else {
            // A blank line is the end of a group of people and we can add the
            // group to the groups.
            groups.push(&group);
            group = Default::default();
        }
    }

    groups.push(&group);
    groups
}

fn part_one(input: &str) {
    let questions = input_to_part_one_questions(&input);
    let total: usize = questions.iter().sum();

    println!("Part 1: Yes = {}", total);
}

fn part_two(input: &str) {
    let groups = input_to_part_two_questions(&input);
    let count = groups.same_answers();

    println!("Part 2: {}", count);
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = env::args().collect();

    // Get the input
    let mut buffer = String::new();
    let mut reader = input_reader(args)?;
    reader.read_to_string(&mut buffer)?;


    part_one(&buffer);
    part_two(&buffer);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
}
