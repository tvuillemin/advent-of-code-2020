use std::fs::File;
use std::io::prelude::Read;

struct BoardingPass {
    seat_id: u32,
}

impl BoardingPass {
    fn new(code: &str) -> BoardingPass {
        let mut lower_row = 0;
        let mut upper_row = 127;
        let mut lower_column = 0;
        let mut upper_column = 7;
        for character in code.chars() {
            if character == 'F' {
                upper_row = (lower_row + upper_row) >> 1;
            }
            if character == 'B' {
                lower_row = (lower_row + upper_row + 1) >> 1;
            }
            if character == 'L' {
                upper_column = (lower_column + upper_column) >> 1;
            }
            if character == 'R' {
                lower_column = (lower_column + upper_column + 1) >> 1;
            }
        }

        BoardingPass {
            seat_id: lower_row * 8 + lower_column,
        }
    }
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

fn main() {
    let boarding_passes: Vec<BoardingPass> = get_lines()
        .iter()
        .map(|line| BoardingPass::new(line))
        .collect();

    let max_seat_id = boarding_passes
        .iter()
        .map(|boarding_pass| boarding_pass.seat_id)
        .max()
        .unwrap();
    println!("Part 1: {}", max_seat_id);

    let mut seat_ids: Vec<u32> = boarding_passes
        .iter()
        .map(|boarding_pass| boarding_pass.seat_id)
        .collect();
    seat_ids.sort();
    for (index, seat_id) in seat_ids.iter().enumerate() {
        if 0 < index && index < seat_ids.len() - 1 {
            let previous_seat_id = seat_ids[index - 1];
            if seat_id != &(previous_seat_id + 1) {
                println!("Part 2: {}", seat_id - 1);
            }
        }
    }
}
