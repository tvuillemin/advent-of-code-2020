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

struct Group {
    answers: Vec<HashSet<char>>,
}

impl Group {
    fn new(line_group: &[&str]) -> Group {
        let mut answers: Vec<HashSet<char>> = Vec::new();

        for line in line_group.iter() {
            let mut set: HashSet<char> = HashSet::new();
            for char in line.chars() {
                set.insert(char);
            }
            answers.push(set);
        }

        Group { answers: answers }
    }

    fn anyone_answers(&self) -> HashSet<char> {
        let mut anyone_answers: HashSet<char> = HashSet::new();
        for someone_answers in self.answers.iter() {
            anyone_answers.extend(someone_answers);
        }
        anyone_answers
    }

    fn everyone_answers(&self) -> HashSet<char> {
        let mut everyone_answers: HashSet<char> = self.answers[0].clone();
        for someone_answers in self.answers[1..].iter() {
            everyone_answers = everyone_answers
                .intersection(someone_answers)
                .cloned()
                .collect::<HashSet<char>>();
        }
        everyone_answers
    }
}

fn main() {
    let mut groups: Vec<Group> = Vec::new();
    let mut line_group: Vec<&str> = Vec::new();

    for line in get_lines().iter() {
        if line == "" {
            groups.push(Group::new(&line_group));
            line_group.clear();
        } else {
            line_group.push(line);
        }
    }

    println!(
        "Part 1: {}",
        groups
            .iter()
            .map(|group| group.anyone_answers().len())
            .sum::<usize>()
    );

    println!(
        "Part 2: {}",
        groups
            .iter()
            .map(|group| group.everyone_answers().len())
            .sum::<usize>()
    );
}
