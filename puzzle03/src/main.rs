use std::fs::File;
use std::io::prelude::Read;

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

const LINE_LENGTH: u32 = 31;

fn count_trees(right_moves: u32, down_moves: u32) -> u32 {
    let mut trees_nb = 0;
    let mut x_position = 0;

    for (index, line) in get_lines().iter().enumerate() {
        if (index as u32) % down_moves == 0 {
            let character = get_character(&line, x_position);
            if character == '#' {
                trees_nb += 1;
            }

            x_position = (x_position + right_moves) % LINE_LENGTH;
        }
    }

    trees_nb
}

fn main() {
    println!("{}", count_trees(3, 1));

    println!(
        "{}",
        count_trees(1, 1)
            * count_trees(3, 1)
            * count_trees(5, 1)
            * count_trees(7, 1)
            * count_trees(1, 2)
    )
}
