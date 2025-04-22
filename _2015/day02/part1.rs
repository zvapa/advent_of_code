use _2015_day02::PresentBoxIter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file_content = std::fs::read_to_string("_2015/day02/input1.txt")?;
    let presents = PresentBoxIter::new(input_file_content.lines());
    let mut total_wrapping_paper: u32 = 0;
    for present in presents {
        let present = present?;
        total_wrapping_paper += present.wrap_qty();
    }
    println!("total square feet of wrapping paper: {}", total_wrapping_paper);
    Ok(())
}