#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::fs::File;
use std::io::prelude::Read;

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
}

fn get_character(string: &str, index: u32) -> char {
    let mut character: char = '\0';
    if (index as usize) < string.chars().count() {
        character = string.chars().collect::<Vec<char>>()[index as usize]
    }
    character
}

fn get_lines() -> Vec<String> {
    let mut input_file = File::open("input.txt").unwrap();
    let mut input_data = String::new();
    input_file.read_to_string(&mut input_data).unwrap();

    input_data
        .split('\n')
        .map(|line| String::from(line))
        .collect()
}

fn checks_policy_1(line: &str) -> bool {
    let captures = LINE_REGEX.captures(line).unwrap();

    let min = captures[1].parse::<u32>().unwrap();
    let max = captures[2].parse::<u32>().unwrap();
    let target = captures[3].chars().next().unwrap();
    let password = &captures[4];

    let mut occurences = 0;
    for character in password.chars() {
        if character == target {
            occurences += 1;
        }
    }

    (min <= occurences) && (occurences <= max)
}

fn checks_policy_2(line: &str) -> bool {
    let captures = LINE_REGEX.captures(line).unwrap();

    let index_1 = captures[1].parse::<u32>().unwrap() - 1;
    let index_2 = captures[2].parse::<u32>().unwrap() - 1;
    let target = captures[3].chars().next().unwrap();
    let password = &captures[4];

    let character_1 = get_character(password, index_1);
    let character_2 = get_character(password, index_2);

    (character_1 == target) ^ (character_2 == target)
}

fn main() -> std::io::Result<()> {
    let mut checks_policy_1_nb = 0;
    let mut checks_policy_2_nb = 0;

    for line in get_lines() {
        if checks_policy_1(&line) {
            checks_policy_1_nb += 1;
        }
        if checks_policy_2(&line) {
            checks_policy_2_nb += 1;
        }
    }
    println!("{}", checks_policy_1_nb);
    println!("{}", checks_policy_2_nb);
    Ok(())
}
