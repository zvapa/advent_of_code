use _2015_day05::{
    contains_any_two_letters_pair_twice_without_overlapping,
    contains_repeating_letter_with_one_between,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file_content = std::fs::read_to_string("_2015/day05/input.txt")?;
    let result = input_file_content
        .lines()
        .filter(|line| {
            if contains_any_two_letters_pair_twice_without_overlapping(line)
                && contains_repeating_letter_with_one_between(line)
            {
                true
            } else {
                false
            }
        })
        .count();
    println!("{result}");
    Ok(())
}
