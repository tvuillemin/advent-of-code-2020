#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::Read;

fn get_lines() -> Vec<String> {
    let mut input_file = File::open("input.txt").unwrap();
    let mut input_data = String::new();
    input_file.read_to_string(&mut input_data).unwrap();

    input_data
        .split('\n')
        .map(|line| String::from(line))
        .collect()
}

lazy_static! {
    static ref INSTRUCTION_RE: Regex = Regex::new(r"^(nop|acc|jmp) ([\+-]\d+)$").unwrap();
}

struct Instruction {
    operation: String,
    argument: i32,
}

impl Instruction {
    fn new(line: &str) -> Instruction {
        let captures = INSTRUCTION_RE.captures(line).unwrap();

        Instruction {
            operation: String::from(&captures[1]),
            argument: captures[2].parse::<i32>().unwrap(),
        }
    }
}

struct BootLoader {
    instructions: Vec<Instruction>,
    current_pointer: i32,
    accumulator: i32,
    already_run_indices: HashSet<i32>,
}

impl BootLoader {
    fn new(instructions: Vec<Instruction>) -> BootLoader {
        BootLoader {
            instructions: instructions,
            current_pointer: 0,
            accumulator: 0,
            already_run_indices: HashSet::new(),
        }
    }

    fn boot(&mut self) {
        let mut booting: bool = true;
        while booting {
            self.already_run_indices.insert(self.current_pointer);
            let instruction = &self.instructions[self.current_pointer as usize];

            if instruction.operation == "acc" {
                self.accumulator += instruction.argument;
                self.current_pointer += 1;
            } else if instruction.operation == "jmp" {
                self.current_pointer += instruction.argument;
            } else if instruction.operation == "nop" {
                self.current_pointer += 1;
            }

            if self.already_run_indices.contains(&self.current_pointer) {
                println!("Part 1: {}", self.accumulator);
                booting = false;
            }
        }
    }
}

fn main() {
    let instructions: Vec<Instruction> = get_lines()
        .iter()
        .map(|line| Instruction::new(line))
        .collect();

    let mut bootloader = BootLoader::new(instructions);
    bootloader.boot();
}
