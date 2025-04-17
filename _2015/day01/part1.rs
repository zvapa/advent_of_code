use _2015_day01::DirectionIter;
use _2015_day01::FloorTracker;
use _2015_day01::parse_directions_from_str;
use _2015_day01::traverse_building;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file_content = std::fs::read_to_string("_2015/day01/input1.txt")?;
    let directions: DirectionIter<_> = parse_directions_from_str(&input_file_content);
    let mut floor_tracker = FloorTracker::new();
    traverse_building(&mut floor_tracker, directions);
    println!("{:?}", floor_tracker.current_floor());
    Ok(())
}
