use std::collections::HashSet;
use std::error::Error;

use _2015_day03::Grid;
use _2015_day03::HouseLocation;
use _2015_day03::Santa;
use _2015_day03::deliver_presents_santa_and_robot;

fn main() -> Result<(), Box<dyn Error>> {
    let start = HouseLocation { x: 0, y: 0 };
    let mut santa = Santa {
        current_location: start,
    };
    let mut robot_santa = Santa {
        current_location: start,
    };
    let mut houses_with_presents: Grid = HashSet::new();
    houses_with_presents.insert(start);

    let input_file_content = std::fs::read_to_string("_2015/day03/input.txt")?;
    deliver_presents_santa_and_robot(
        &mut santa,
        &mut robot_santa,
        &mut houses_with_presents,
        input_file_content,
    )?;

    println!("{}", houses_with_presents.len());

    Ok(())
}
