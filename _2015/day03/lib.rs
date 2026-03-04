use std::{collections::HashSet, str::Chars};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct HouseLocation {
    pub x: i32,
    pub y: i32,
}

pub type Grid = HashSet<HouseLocation>;

pub enum SantaDestinationTravel {
    North { from: HouseLocation },
    South { from: HouseLocation },
    West { from: HouseLocation },
    East { from: HouseLocation },
}

impl SantaDestinationTravel {
    pub fn travel(&self) -> HouseLocation {
        match &self {
            SantaDestinationTravel::North { from } => HouseLocation {
                x: from.x,
                y: from.y + 1,
            },
            SantaDestinationTravel::South { from } => HouseLocation {
                x: from.x,
                y: from.y - 1,
            },
            SantaDestinationTravel::West { from } => HouseLocation {
                x: from.x - 1,
                y: from.y,
            },
            SantaDestinationTravel::East { from } => HouseLocation {
                x: from.x + 1,
                y: from.y,
            },
        }
    }
}

pub struct TravelingSanta {
    pub current_location: HouseLocation,
}

impl TravelingSanta {
    pub fn deliver_present(&mut self, direction: char) -> Result<(), String> {
        // north (^), south (v), east (>), or west (<)
        let destination: SantaDestinationTravel = match direction {
            '^' => SantaDestinationTravel::North {
                from: self.current_location,
            },
            'v' => SantaDestinationTravel::South {
                from: self.current_location,
            },
            '>' => SantaDestinationTravel::East {
                from: self.current_location,
            },
            '<' => SantaDestinationTravel::West {
                from: self.current_location,
            },
            unknown => return Err(format!("unknown destination: {}", unknown)),
        };
        self.current_location = destination.travel();
        Ok(())
    }
}

fn deliver_and_mark(
    santa: &mut TravelingSanta,
    houses_with_presents: &mut Grid,
    char: char,
) -> Result<(), String> {
    santa.deliver_present(char)?;
    houses_with_presents.insert(santa.current_location);
    Ok(())
}

pub fn deliver_presents_single_santa(
    santa: &mut TravelingSanta,
    houses_with_presents: &mut Grid,
    input_file_content: String,
) -> Result<(), String> {
    Ok(for char in input_file_content.chars() {
        deliver_and_mark(santa, houses_with_presents, char)?;
    })
}

pub struct PresentRoute<'a> {
    input: Chars<'a>,
    turn: usize,
}

impl<'a> PresentRoute<'a> {
    pub fn new(input: Chars<'a>) -> Self {
        Self { input, turn: 0 }
    }
}

impl<'a> Iterator for PresentRoute<'a> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        self.input.next().map(|c| {
            let agent_id = self.turn % 2;
            self.turn += 1;
            (agent_id, c)
        })
    }
}

pub fn deliver_presents_santa_and_robot(
    santa: &mut TravelingSanta,
    robot_santa: &mut TravelingSanta,
    houses_with_presents: &mut Grid,
    input_file_content: String,
) -> Result<(), String> {
    let present_route = PresentRoute::new(input_file_content.chars());
    for (agent_id, direction) in present_route {
        let which_santa = if agent_id % 2 == 0 {
            &mut *santa
        } else {
            &mut *robot_santa
        };
        deliver_and_mark(which_santa, houses_with_presents, direction)?
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deliver_presents_single_santa() {
        let mut santa = TravelingSanta {
            current_location: HouseLocation { x: 0, y: 0 },
        };
        let mut houses_with_presents: Grid = std::collections::HashSet::new();
        houses_with_presents.insert(santa.current_location);
        let directions = ">".to_string();
        let _ = deliver_presents_single_santa(&mut santa, &mut houses_with_presents, directions);

        assert_eq!(houses_with_presents.len(), 2)
    }

    #[test]
    fn test_deliver_presents_santa_and_robot() {
        let start = HouseLocation { x: 0, y: 0 };
        let mut santa = TravelingSanta {
            current_location: start,
        };
        let mut robot_santa = TravelingSanta {
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
}
