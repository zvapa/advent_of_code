use _2015_day02::PresentBoxIter;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let input_file_content = std::fs::read_to_string("_2015/day02/input1.txt")?;
    let present_box_iter = PresentBoxIter::new(input_file_content.lines());
    let mut total_ribbon: u32 = 0;
    for present in present_box_iter {
        let present = present?;
        total_ribbon += present.ribbon_qty()
    }
    println!("total feet of ribbon: {}", total_ribbon);
    Ok(())
}