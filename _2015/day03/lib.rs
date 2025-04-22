pub type Grid = std::collections::HashSet<HouseLocation>;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct HouseLocation {
    pub x: i32,
    pub y: i32,
}

pub enum SantaDestination {
    North(HouseLocation),
    South(HouseLocation),
    West(HouseLocation),
    East(HouseLocation),
}

impl SantaDestination {
    pub fn travel(&self) -> HouseLocation {
        match &self {
            SantaDestination::North(current_location) => HouseLocation {
                x: current_location.x,
                y: current_location.y + 1,
            },
            SantaDestination::South(current_location) => HouseLocation {
                x: current_location.x,
                y: current_location.y - 1,
            },
            SantaDestination::West(current_location) => HouseLocation {
                x: current_location.x - 1,
                y: current_location.y,
            },
            SantaDestination::East(current_location) => HouseLocation {
                x: current_location.x + 1,
                y: current_location.y,
            },
        }
    }
}

pub struct Santa {
    pub current_location: HouseLocation,
}

impl Santa {
    pub fn deliver_present(&mut self, direction: char) -> Result<(), String> {
        // north (^), south (v), east (>), or west (<)
        let destination: SantaDestination = match direction {
            '^' => SantaDestination::North(self.current_location),
            'v' => SantaDestination::South(self.current_location),
            '>' => SantaDestination::East(self.current_location),
            '<' => SantaDestination::West(self.current_location),
            unknown => return Err(format!("unknown destination: {}", unknown)),
        };
        self.current_location = destination.travel();
        Ok(())
    }
}

fn deliver_and_mark(
    santa: &mut Santa,
    houses_with_presents: &mut Grid,
    char: char,
) -> Result<(), String> {
    santa.deliver_present(char)?;
    houses_with_presents.insert(santa.current_location);
    Ok(())
}

pub fn deliver_presents_single_santa(
    santa: &mut Santa,
    houses_with_presents: &mut Grid,
    input_file_content: String,
) -> Result<(), String> {
    Ok(for char in input_file_content.chars() {
        deliver_and_mark(santa, houses_with_presents, char)?;
    })
}

pub fn deliver_presents_santa_and_robot(
    santa: &mut Santa,
    robot_santa: &mut Santa,
    houses_with_presents: &mut Grid,
    input_file_content: String,
) -> Result<(), String> {
    let mut directions_iter = input_file_content.chars();
    loop {
        match directions_iter.next() {
            Some(dir) => deliver_and_mark(santa, houses_with_presents, dir)?,
            None => break,
        }
        match directions_iter.next() {
            Some(dir) => deliver_and_mark(robot_santa, houses_with_presents, dir)?,
            None => break,
        }
    }
    Ok(())
}

#[test]
fn test_deliver_presents_single_santa() {
    let mut santa = Santa {
        current_location: HouseLocation { x: 0, y: 0 },
    };
    let mut houses_with_presents: Grid = std::collections::HashSet::new();
    houses_with_presents.insert(santa.current_location);
    let map = ">".to_string();
    let _ = deliver_presents_single_santa(&mut santa, &mut houses_with_presents, map);

    assert_eq!(houses_with_presents.len(), 2)
}

#[test]
fn test_deliver_presents_santa_and_robot() {
    let start = HouseLocation { x: 0, y: 0 };
    let mut santa = Santa {
        current_location: start,
    };
    let mut robot_santa = Santa {
        current_location: start,
    };
    let mut houses_with_presents: Grid = std::collections::HashSet::new();
    houses_with_presents.insert(start);
    let map = String::from("^>v<");
    let _ = deliver_presents_santa_and_robot(
        &mut santa,
        &mut robot_santa,
        &mut houses_with_presents,
        map,
    );
    assert_eq!(houses_with_presents.len(), 3)
}
