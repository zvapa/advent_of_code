use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}
impl TryFrom<&str> for Instruction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "turn on" => Ok(Instruction::TurnOn),
            "turn off" => Ok(Instruction::TurnOff),
            "toggle" => Ok(Instruction::Toggle),
            _ => Err("Unknown instruction".to_string()),
        }
    }
}

#[derive(PartialEq, Debug, Default)]
pub struct Area {
    pub from: (u16, u16),
    pub to: (u16, u16),
}

#[derive(PartialEq, Debug)]
pub struct GridWork {
    pub area: Area,
    pub action: Instruction,
}

pub struct Brightness(pub u16);

pub struct Grid(HashMap<(u16, u16), Brightness>);

impl Grid {
    pub fn new() -> Grid {
        let mut grid = HashMap::<(u16, u16), Brightness>::new();
        for x in 0..1000 {
            for y in 0..1000 {
                grid.insert((x, y), Brightness(0));
            }
        }
        Grid(grid)
    }

    pub fn apply<Translation>(
        &mut self,
        grid_work: &GridWork,
        effect: Translation,
    ) -> Result<(), String>
    where
        Translation: Fn(&Instruction, &mut Brightness),
    {
        match grid_work.area {
            Area {
                from: (x0, y0),
                to: (x1, y1),
            } if x0 <= x1 && y0 <= y1 => {
                for x in x0..(x1 + 1) {
                    for y in y0..(y1 + 1) {
                        let brightness: &mut Brightness =
                            self.0.get_mut(&(x, y)).expect("this should not happen");
                        effect(&grid_work.action, brightness)
                    }
                }
                Ok(())
            }
            _ => Err("could not understand instruction".to_string()),
        }
    }

    pub fn total_brightness(&self) -> u32 {
        self.0.values().map(|l| l.0 as u32).sum()
    }
}

pub fn parse_instruction(s: &str) -> Result<GridWork, String> {
    let (action_str, area_str) = if let Some(rest) = s.strip_prefix("turn on") {
        ("turn on", rest)
    } else if let Some(rest) = s.strip_prefix("turn off") {
        ("turn off", rest)
    } else if let Some(rest) = s.strip_prefix("toggle") {
        ("toggle", rest)
    } else {
        return Err("could not parse instruction".to_string());
    };

    let action = Instruction::try_from(action_str)?;
    let area = get_area(area_str)?;

    Ok(GridWork { action, area })
}

fn get_area(s: &str) -> Result<Area, String> {
    let mut area: Area = Default::default();
    let mut coord_iter = s.split("through").map(str::trim);
    match coord_iter.next() {
        Some(s) => {
            let mut start_coord_iter = s.split(',').map(str::parse::<u16>);
            match start_coord_iter.next() {
                Some(Ok(x)) => area.from.0 = x,
                Some(Err(e)) => return Err(e.to_string()),
                None => return Err("expected start x coordinate".to_string()),
            };
            match start_coord_iter.next() {
                Some(Ok(y)) => area.from.1 = y,
                Some(Err(e)) => return Err(e.to_string()),
                None => return Err("expected start y coordinate".to_string()),
            };
        },
        None => return Err("expected start coordinates".to_string()),
    };

    match coord_iter.next() {
        Some(s) => {
            let mut end_coord_iter = s.split(',').map(str::parse::<u16>);
            match end_coord_iter.next() {
                Some(Ok(x)) => area.to.0 = x,
                Some(Err(e)) => return Err(e.to_string()),
                None => return Err("expected end x coordinate".to_string()),
            };
            match end_coord_iter.next() {
                Some(Ok(y)) => area.to.1 = y,
                Some(Err(e)) => return Err(e.to_string()),
                None => return Err("expected end y coordinate".to_string()),
            };
        },
        None => return Err("expected end coordinates".to_string()),
    };

    match coord_iter.next() {
        Some(_) => Err("invalid coordinates".to_string()),
        None => Ok(area),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_area() {
        let valid_input = "660,55 through 986,197";
        let invalid_input = "660,55 through ";
        assert_eq!(
            get_area(valid_input),
            Ok(Area {
                from: (660, 55),
                to: (986, 197)
            })
        );
        assert!(get_area(invalid_input).is_err());
    }

    #[test]
    fn test_parse_instruction() {
        let valid_input = "turn off 499,499 through 500,500";
        assert_eq!(
            parse_instruction(valid_input),
            Ok(GridWork {
                area: Area {
                    from: (499, 499),
                    to: (500, 500)
                },
                action: Instruction::TurnOff
            })
        )
    }
}
