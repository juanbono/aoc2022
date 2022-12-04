use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let input_file = File::open("inputs/input1")?;
    let reader = BufReader::new(input_file);
    let mut current_elf = 0;
    let mut elves_calories: Vec<u32> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if !line.is_empty() {
            let parsed_calories: u32 = line.parse().unwrap();
            current_elf += parsed_calories;
        } else {
            elves_calories.push(current_elf);
            current_elf = 0;
        }
    }

    elves_calories.sort();
    elves_calories.reverse();
    let max_calories_elf = elves_calories.iter().max().unwrap();
    println!("{max_calories_elf}");
    let winner_elf = elves_calories.binary_search(max_calories_elf).unwrap();
    println!("part 1: {winner_elf}");
    println!(
        "part 2: {:?}",
        elves_calories[0] + elves_calories[1] + elves_calories[2]
    );
    Ok(())
}
