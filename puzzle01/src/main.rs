use std::fs::File;
use std::io::prelude::Read;

fn main() -> std::io::Result<()> {
    let mut input_file = File::open("input.txt")?;
    let mut input_data = String::new();
    input_file.read_to_string(&mut input_data)?;
    let input_values: Vec<u32> = input_data
        .split('\n')
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    for i in input_values.iter() {
        for j in input_values.iter() {
            for k in input_values.iter() {
                if i + j + k == 2020 {
                    println!("{}", i * j * k);
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}
