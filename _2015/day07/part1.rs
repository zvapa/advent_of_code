use _2015_day07::Instruction;
use _2015_day07::SignalSource;
use _2015_day07::a_found;
use _2015_day07::calculate_signals_until;
use _2015_day07::circuit_map;
use _2015_day07::parse_instructions;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_content = std::fs::read_to_string("_2015/day07/input.txt")?;
    let instructions: Vec<Instruction<'_>> = parse_instructions(&file_content)?;
    let mut circuit: HashMap<&str, SignalSource<'_>> = circuit_map(instructions);

    // loop until we find a signal value for "a"
    calculate_signals_until(&mut circuit, a_found);

    // let mut sorted_pairs: Vec<(&&str, &SignalSource)> = a_found.iter().collect();
    // sorted_pairs.sort_by_key(|(key, _)| *key);
    // for (k, v) in sorted_pairs {
    //     println!("{}: {}", k, v);
    // }

    println!("{}", circuit.get("a").unwrap());

    Ok(())
}
