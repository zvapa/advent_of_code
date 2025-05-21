use _2015_day07::{
    SignalSource, a_found, calculate_signals_until, circuit_map, parse_instructions,
};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_content = std::fs::read_to_string("_2015/day07/input.txt")?;
    let instructions = parse_instructions(&file_content)?;
    let mut circuit: HashMap<&str, SignalSource<'_>> = circuit_map(instructions);
    let mut original_circuit: HashMap<&str, SignalSource<'_>> = circuit.clone();
    calculate_signals_until(&mut circuit, a_found);

    // override wire "b" to signal from "a"
    let a_signal = circuit.get("a").unwrap();
    original_circuit
        .entry("b")
        .and_modify(|signal| *signal = *a_signal);

    calculate_signals_until(&mut original_circuit, a_found);
    println!("{}", original_circuit.get("a").unwrap());

    Ok(())
}
