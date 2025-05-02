use _2015_day05::{
    contains_at_least_one_letter_that_appears_twice_in_a_row, contains_at_least_three_vowels,
    does_not_contain_excluded_strings,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file_content = std::fs::read_to_string("_2015/day05/input.txt")?;
    let result = input_file_content
        .lines()
        .filter(|line| {
            contains_at_least_three_vowels(line)
                && contains_at_least_one_letter_that_appears_twice_in_a_row(line)
                && does_not_contain_excluded_strings(line)
        })
        .count();
    println!("{result}");
    Ok(())
}
