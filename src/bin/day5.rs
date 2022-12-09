extern crate pest;
#[macro_use]
extern crate pest_derive;

use itertools::Itertools;
use pest::Parser;
use std::collections::HashMap;
use std::str::FromStr;
use std::{fs, io};

#[derive(Parser)]
#[grammar = "grammars/day5.pest"]
pub struct Day5Parser;

#[derive(Debug, PartialEq, Default)]
struct State {
    stacks: HashMap<usize, Line>,
}

impl State {
    pub fn apply(&mut self, instruction: &Move) {
        let amount = instruction.amount;
        let mut moved_crates = vec![];
        self.stacks
            .entry(instruction.from - 1)
            .and_modify(|from_line| {
                moved_crates = from_line.remove_crates(amount);
            });

        self.stacks
            .entry(instruction.to - 1)
            .and_modify(|to_line| to_line.add_crates(moved_crates));
    }

    pub fn apply2(&mut self, instruction: &Move) {
        let amount = instruction.amount;
        let mut moved_crates = vec![];
        self.stacks
            .entry(instruction.from - 1)
            .and_modify(|from_line| {
                moved_crates = from_line.remove_crates(amount);
            });

        self.stacks
            .entry(instruction.to - 1)
            .and_modify(|to_line| to_line.add_crates2(moved_crates));
    }

    pub fn from_pair(pair: pest::iterators::Pair<Rule>) -> Self {
        let mut stacks: HashMap<usize, Line> = HashMap::new();
        for elem in pair.into_inner() {
            // parsing each line
            match elem.as_rule() {
                // parsing a single line
                Rule::line => {
                    for (index, inner_elem) in elem.into_inner().enumerate() {
                        // populate the lines for each column.
                        if !stacks.contains_key(&index) {
                            stacks.insert(index, Line::default());
                        }
                        let item = inner_elem.into_inner().next().unwrap();
                        match item.as_rule() {
                            Rule::empty_crate => {
                                stacks.entry(index).and_modify(|line| line.add(None));
                            }
                            Rule::full_crate => {
                                let letter = item.into_inner().next().unwrap().as_str().to_owned();
                                stacks
                                    .entry(index)
                                    .and_modify(|line| line.add(Some(letter)));
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                Rule::column_numbers => {}
                _ => unreachable!(),
            }
        }
        State { stacks }
    }
}

#[derive(Debug, PartialEq, Default)]
struct Line(Vec<Option<String>>);

impl Line {
    pub fn get_last_crate(&self) -> Option<String> {
        self.0
            .clone()
            .into_iter()
            .filter(|item| item.is_some())
            .next()
            .unwrap()
    }
    pub fn add(&mut self, item: Option<String>) {
        if let Some(_) = item {
            self.0.push(item)
        }
    }

    pub fn add_crates(&mut self, items: Vec<Option<String>>) {
        for item in items {
            self.0.insert(0, item)
        }
    }

    pub fn add_crates2(&mut self, items: Vec<Option<String>>) {
        let mut new_items = items.clone();
        let mut old_items = self.0.clone();
        new_items.append(&mut old_items);
        self.0 = new_items;
    }

    pub fn remove_crates(&mut self, amount: usize) -> Vec<Option<String>> {
        let removed = self.0.drain(0..amount).collect();
        removed
    }
}

#[derive(Debug, PartialEq)]
struct Move {
    from: usize,
    to: usize,
    amount: usize,
}

#[derive(Debug, PartialEq, Default)]
struct Program(Vec<Move>);

impl Program {
    fn from_pair(pair: pest::iterators::Pair<Rule>) -> Program {
        let mut moves: Vec<Move> = vec![];
        for elem in pair.into_inner() {
            let mut members = elem.into_inner();
            let amount = members.next().unwrap().as_str().parse().unwrap();
            let from = members.next().unwrap().as_str().parse().unwrap();
            let to = members.next().unwrap().as_str().parse().unwrap();
            let mov = Move { from, to, amount };
            moves.push(mov);
        }

        Program(moves)
    }
}

#[derive(Debug, PartialEq, Default)]
struct Problem {
    state: State,
    moves: Program,
}

impl Problem {
    pub fn run(&mut self) -> String {
        for instruction in &self.moves.0 {
            self.state.apply(instruction);
        }

        self.last_crates()
    }

    pub fn run2(&mut self) -> String {
        for instruction in &self.moves.0 {
            self.state.apply2(instruction);
        }

        self.last_crates()
    }

    pub fn last_crates(&self) -> String {
        let mut crates = String::new();

        let stack_numbers = self.state.stacks.keys().sorted();

        for n in stack_numbers {
            let line = self.state.stacks.get(n).unwrap();
            if let Some(character) = line.get_last_crate() {
                crates.extend(character.chars())
            }
        }

        crates
    }
}

impl FromStr for Problem {
    type Err = pest::error::Error<Rule>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parsed_program = Day5Parser::parse(Rule::input, s)?;
        let mut problem = Problem::default();
        if let Some(parsed_input) = parsed_program.next() {
            for elem in parsed_input.into_inner() {
                match elem.as_rule() {
                    Rule::state => {
                        let state = State::from_pair(elem);
                        problem.state = state;
                    }

                    Rule::program => {
                        let program = Program::from_pair(elem);
                        problem.moves = program;
                    }
                    _ => unreachable!(),
                }
            }
        }
        Ok(problem)
    }
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("inputs/input5")?;
    let mut problem: Problem = input.parse().unwrap();
    problem.run();

    println!("Part 1: {}", problem.last_crates());


    let mut problem: Problem = input.parse().unwrap();
    problem.run2();

    println!("Part 2: {}", problem.last_crates());


    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Problem;

    #[test]
    fn can_parse_problem() {
        let program = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

        assert!(program.parse::<Problem>().is_ok());
    }

    #[test]
    fn can_run_problem() {
        let program = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

        let mut problem = program.parse::<Problem>().unwrap();
        problem.run();
        // dbg!(&problem.state);
        assert_eq!("CMZ".to_owned(), problem.last_crates());
    }
}
