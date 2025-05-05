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
        .try_for_each(|i| grid.apply(&i, light_up))?;

    println!("total lights: {}", grid.total_brightness());

    let elapsed: Duration = start.elapsed();
    println!("{elapsed:?}");

    Ok(())
}

fn light_up(instruction: &Instruction, brightness: &mut Brightness) {
    match instruction {
        Instruction::TurnOn => *brightness = Brightness(1),
        Instruction::TurnOff => *brightness = Brightness(0),
        Instruction::Toggle => match brightness {
            Brightness(1) => {
                *brightness = Brightness(0);
            }
            Brightness(0) => {
                *brightness = Brightness(1);
            }
            _ => (),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::light_up;
    use _2015_day06::{Area, Grid, GridWork, Instruction};

    #[test]
    fn test_grid_apply_instruction() {
        let mut grid = Grid::new();
        let grid_work = GridWork {
            area: Area {
                from: (0, 0),
                to: (999, 999),
            },
            action: Instruction::TurnOn,
        };
        grid.apply(&grid_work, light_up).unwrap();
        assert_eq!(grid.total_brightness(), 1_000_000)
    }
}
