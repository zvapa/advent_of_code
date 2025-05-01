const DELIMITER: char = 'x';

impl<'a> TryFrom<&'a str> for PresentBox {
    type Error = String;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        let (dim1, dim2, dim3) = split_into_three(line)?;
        let dim1 = dim1
            .parse::<u32>()
            .map_err(|e| format!("failed to parse {} into u32: {}", dim1, e))?;
        let dim2 = dim2
            .parse::<u32>()
            .map_err(|e| format!("failed to parse {} into u32: {}", dim2, e))?;
        let dim3 = dim3
            .parse::<u32>()
            .map_err(|e| format!("failed to parse {} into u32: {}", dim3, e))?;
        Ok(PresentBox::new([dim1, dim2, dim3]))
    }
}

trait SplitOnceNonEmpty<'a> {
    fn split_once_non_empty(&self, delimiter: char) -> Option<(&'a str, &'a str)>;
}
impl<'a> SplitOnceNonEmpty<'a> for &'a str {
    fn split_once_non_empty(&self, delimiter: char) -> Option<(&'a str, &'a str)> {
        match self.split_once(delimiter) {
            Some((prefix, suffix)) => {
                if prefix.is_empty() || suffix.is_empty() {
                    return None;
                } else {
                    return Some((prefix, suffix));
                }
            }
            None => todo!(),
        }
    }
}

fn split_into_three(line: &str) -> Result<(&str, &str, &str), String> {
    let split = line
        .trim()
        .split_once_non_empty(DELIMITER)
        .and_then(|(part1, rest)| {
            rest.split_once_non_empty(DELIMITER)
                .map(|(part2, part3)| (part1, part2, part3))
        });
    let err = format!("Failed to split in exactly 3 parts: {line}");
    match split {
        Some(t) => Ok(t),
        None => Err(err),
    }
}

#[derive(Debug, PartialEq)]
pub struct PresentBox {
    /// length, width, height, in ascending order
    ordered_dimensions: [u32; 3],
}
impl PresentBox {
    fn new(mut dimensions: [u32; 3]) -> PresentBox {
        dimensions.sort();
        PresentBox {
            ordered_dimensions: dimensions,
        }
    }
    fn area(&self) -> u32 {
        self.ordered_dimensions[0] * self.ordered_dimensions[1] * 2
            + self.ordered_dimensions[1] * self.ordered_dimensions[2] * 2
            + self.ordered_dimensions[0] * self.ordered_dimensions[2] * 2
    }
    fn smallest_side_area(&self) -> u32 {
        self.ordered_dimensions[0] * self.ordered_dimensions[1]
    }
    fn smallest_perimeter(&self) -> u32 {
        (self.ordered_dimensions[0] + self.ordered_dimensions[1]) * 2
    }
    fn volume(&self) -> u32 {
        self.ordered_dimensions[0] * self.ordered_dimensions[1] * self.ordered_dimensions[2]
    }
    pub fn wrap_qty(&self) -> u32 {
        self.area() + self.smallest_side_area()
    }
    pub fn ribbon_qty(&self) -> u32 {
        self.smallest_perimeter() + self.volume()
    }
}

pub struct PresentBoxIter<'a, I: Iterator<Item = &'a str>> {
    lines: I,
}

impl<'a, I: Iterator<Item = &'a str>> PresentBoxIter<'a, I> {
    pub fn new(dimensions_in_order: I) -> Self {
        Self {
            lines: dimensions_in_order,
        }
    }
}

impl<'a, I> Iterator for PresentBoxIter<'a, I>
where
    I: Iterator<Item = &'a str>,
{
    type Item = Result<PresentBox, String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            Some(line) => Some(PresentBox::try_from(line)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_into_three_valid_line() {
        let expected = split_into_three("21x2x22");
        assert_eq!(expected, Ok(("21", "2", "22")))
    }

    #[test]
    fn test_split_into_three_invalid_line_nan() {
        let result = dbg!(split_into_three("21x2x"));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed"));
    }

    #[test]
    fn test_try_from_valid_numbers() {
        let result = PresentBox::try_from("21x2x22").unwrap();
        assert_eq!(
            result,
            PresentBox {
                ordered_dimensions: [2, 21, 22]
            }
        )
    }

    #[test]
    fn test_area_calculation() {
        let present = PresentBox::new([2, 3, 4]);
        assert_eq!(present.area(), 52)
    }

    #[test]
    fn test_present_wrap_qty_calculation() {
        let present1 = PresentBox::new([2, 3, 4]);
        let present2 = PresentBox::new([1, 1, 10]);
        assert_eq!(present1.wrap_qty(), 58);
        assert_eq!(present2.wrap_qty(), 43);
    }

    #[test]
    fn test_present_ribbon_qty_calculation() {
        let present1 = PresentBox::new([2, 3, 4]);
        let present2 = PresentBox::new([1, 1, 10]);
        assert_eq!(present1.ribbon_qty(), 34);
        assert_eq!(present2.ribbon_qty(), 14);
    }
}
