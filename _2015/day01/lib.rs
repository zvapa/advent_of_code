#[derive(PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
}

impl std::convert::TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Direction::Up),
            ')' => Ok(Direction::Down),
            _ => Err(format!("Invalid direction character: {}", value)),
        }
    }
}

pub struct FloorTracker {
    floor: i32,
    steps_taken: u32,
}

impl FloorTracker {
    pub fn new() -> FloorTracker {
        FloorTracker {
            floor: 0,
            steps_taken: 0,
        }
    }
    fn next(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.floor += 1,
            Direction::Down => self.floor -= 1,
        }
        self.steps_taken += 1
    }
    pub fn current_floor(&self) -> i32 {
        self.floor
    }
    pub fn steps_taken(&self) -> u32 {
        self.steps_taken
    }
}

pub struct DirectionIter<I: Iterator<Item = char>> {
    chars: I,
}

impl<I: Iterator<Item = char>> DirectionIter<I> {
    fn new(iter: I) -> Self {
        Self { chars: iter }
    }
}

impl Iterator for DirectionIter<std::str::Chars<'_>> {
    type Item = Result<Direction, String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.next() {
            Some(c) => Some(Direction::try_from(c)),
            None => None,
        }
    }
}

pub fn parse_directions_from_str(input: &str) -> DirectionIter<std::str::Chars> {
    DirectionIter::new(input.chars())
}

pub fn traverse_building(
    floor_tracker: &mut FloorTracker,
    directions: impl Iterator<Item = Result<Direction, String>>,
) {
    for direction in directions {
        floor_tracker.next(direction.expect("could not process direction"));
    }
}

pub fn stop_at_basement(
    floor_tracker: &mut FloorTracker,
    directions: impl Iterator<Item = Result<Direction, String>>,
) -> Option<u32> {
    for direction in directions {
        floor_tracker.next(direction.expect("could not process direction"));
        if floor_tracker.floor == -1 {
            return Some(floor_tracker.steps_taken);
        }
    }
    None
}

#[test]
fn test_floor_tracker_up_and_down() {
    let mut tracker = FloorTracker::new();
    let directions = parse_directions_from_str("(())");
    traverse_building(&mut tracker, directions);
    assert_eq!(tracker.floor, 0);
}

#[test]
fn test_stop_at_basement_reached() {
    let mut tracker = FloorTracker::new();
    let directions = parse_directions_from_str("())");
    let steps_to_basement = stop_at_basement(&mut tracker, directions);
    assert_eq!(steps_to_basement, Some(3));
}

#[test]
fn test_stop_at_basement_not_reached() {
    let mut tracker = FloorTracker::new();
    let directions = parse_directions_from_str("((())");
    let steps_to_basement = stop_at_basement(&mut tracker, directions);
    assert_eq!(steps_to_basement, None);
}
