use _2015_day06::{Brightness, Grid, GridWork, Instruction, parse_instruction};
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    let input_file_content = std::fs::read_to_string("_2015/day06/input.txt")?;
    let mut grid = Grid::new();
    input_file_content
        .lines()
        .map(|l| parse_instruction(l))
        .collect::<Result<Vec<GridWork>, String>>()?
        .into_iter()
        .try_for_each(|i| grid.apply(&i, adjust_brightness))?;

    println!("total lights: {}", grid.total_brightness());

    let elapsed: Duration = start.elapsed();
    println!("{elapsed:?}");

    Ok(())
}

fn adjust_brightness(instruction: &Instruction, brightness: &mut Brightness) {
    match instruction {
        Instruction::TurnOn => brightness.0 += 1,
        Instruction::TurnOff => brightness.0 = brightness.0.saturating_sub(1),
        Instruction::Toggle => brightness.0 += 2,
    }
}
