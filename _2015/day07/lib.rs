use std::{
    collections::HashMap,
    fmt::Display,
    ops::{BitAnd, BitOr},
};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum SignalSource<'a> {
    FromValue(u16),
    FromWire {
        wire_id: &'a str,
    },
    FromValueAndGate {
        value: u16,
        wire_id: &'a str,
    },
    FromAndGate {
        wire_id1: &'a str,
        wire_id2: &'a str,
    },
    FromValueOrGate {
        value: u16,
        wire_id: &'a str,
    },
    FromOrGate {
        wire_id1: &'a str,
        wire_id2: &'a str,
    },
    FromLShiftGate {
        wire_id: &'a str,
        value: u16,
    },
    FromRShiftGate {
        wire_id: &'a str,
        value: u16,
    },
    FromNotGate {
        wire_id: &'a str,
    },
}

impl<'a> Display for SignalSource<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignalSource::FromValue(value) => {
                write!(f, "{value}")
            }
            SignalSource::FromWire { wire_id } => {
                write!(f, "{wire_id}")
            }
            SignalSource::FromValueAndGate {
                wire_id: wire_id1,
                value,
            } => {
                write!(f, "{value} AND {wire_id1}")
            }
            SignalSource::FromAndGate { wire_id1, wire_id2 } => {
                write!(f, "{wire_id1} AND {wire_id2}")
            }
            SignalSource::FromValueOrGate { value, wire_id } => {
                write!(f, "{value} OR {wire_id}")
            }
            SignalSource::FromOrGate { wire_id1, wire_id2 } => {
                write!(f, "{wire_id1} OR {wire_id2}")
            }
            SignalSource::FromLShiftGate {
                wire_id: wire_id1,
                value,
            } => {
                write!(f, "{wire_id1} LSHIFT {value}")
            }
            SignalSource::FromRShiftGate {
                wire_id: wire_id1,
                value,
            } => {
                write!(f, "{wire_id1} RSHIFT {value}")
            }
            SignalSource::FromNotGate { wire_id } => {
                write!(f, "NOT {}", wire_id)
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Instruction<'a> {
    wire: &'a str,
    source: SignalSource<'a>,
}

impl<'a> Display for Instruction<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "wire: {}, source: {}", self.wire, self.source)
    }
}

// parser for instructions
impl<'a> TryFrom<&'a str> for Instruction<'a> {
    type Error = &'a str;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let wire_err: &'static str = "expected a wire id here";
        let operator_err: &'static str = "expected '->' sign here";
        let number_err: &'static str = "expected a number here";
        let number_parse_err: &'static str = "failed to parse number";
        let incomplete_instruction: &'static str = "incomplete instruction";
        let unknown_token: &'static str = "unknown token";

        let mut split = value.split(' ');
        match split.next() {
            Some(number) if number.starts_with(char::is_numeric) => {
                let val = number.parse::<u16>().map_err(|_| number_parse_err)?;
                match split.next() {
                    Some("AND") => {
                        let wire_operand = split.next().ok_or(wire_err)?;
                        split.next().ok_or(operator_err)?;
                        let wire = split.next().ok_or(wire_err)?;
                        return Ok(Instruction {
                            wire: wire,
                            source: SignalSource::FromValueAndGate {
                                value: val,
                                wire_id: wire_operand,
                            },
                        });
                    }
                    Some("->") => {
                        let wire = split.next().ok_or(wire_err)?;
                        return Ok(Instruction {
                            wire,
                            source: SignalSource::FromValue(val),
                        });
                    }
                    Some(unknown) => {
                        dbg!(unknown);
                        return Err(unknown_token);
                    }
                    None => {
                        return Err(incomplete_instruction)
                    },
                };
            }
            Some("NOT") => {
                let wire_operand = split.next().ok_or(wire_err)?;
                split.next().ok_or(operator_err)?;
                let wire = split.next().ok_or(wire_err)?;
                return Ok(Instruction {
                    wire: wire,
                    source: SignalSource::FromNotGate {
                        wire_id: wire_operand,
                    },
                });
            }
            Some(wire_operand) => {
                match split.next() {
                    Some("AND") => {
                        let wire_operand2 = split.next().ok_or(wire_err)?;
                        split.next().ok_or(operator_err)?;
                        let wire = split.next().ok_or(wire_err)?;
                        return Ok(Instruction {
                            wire: wire,
                            source: SignalSource::FromAndGate {
                                wire_id1: wire_operand,
                                wire_id2: wire_operand2,
                            },
                        });
                    }
                    Some("OR") => {
                        let wire_operand2 = split.next().ok_or(wire_err)?;
                        split.next().ok_or(operator_err)?;
                        let wire = split.next().ok_or(wire_err)?;
                        return Ok(Instruction {
                            wire: wire,
                            source: SignalSource::FromOrGate {
                                wire_id1: wire_operand,
                                wire_id2: wire_operand2,
                            },
                        });
                    }
                    Some("LSHIFT") => {
                        let value_operand = split
                            .next()
                            .ok_or(number_err)?
                            .parse::<u16>()
                            .map_err(|_| number_parse_err)?;
                        split.next().ok_or(operator_err)?;
                        let wire = split.next().ok_or(wire_err)?;
                        return Ok(Instruction {
                            wire: wire,
                            source: SignalSource::FromLShiftGate {
                                wire_id: wire_operand,
                                value: value_operand,
                            },
                        });
                    }
                    Some("RSHIFT") => {
                        let value_operand = split
                            .next()
                            .ok_or(number_err)?
                            .parse::<u16>()
                            .map_err(|_| number_parse_err)?;
                        split.next().ok_or(operator_err)?;
                        let wire = split.next().ok_or(wire_err)?;
                        return Ok(Instruction {
                            wire: wire,
                            source: SignalSource::FromRShiftGate {
                                wire_id: wire_operand,
                                value: value_operand,
                            },
                        });
                    }
                    Some("->") => {
                        let wire = split.next().ok_or(wire_err)?;
                        return Ok(Instruction {
                            wire: wire,
                            source: SignalSource::FromWire {
                                wire_id: wire_operand,
                            },
                        });
                    }
                    Some(unknown) => {
                        dbg!(unknown);
                        return Err(unknown_token);
                    }
                    None => return Err(incomplete_instruction),
                };
            }
            None => {
                return Err("empty instruction");
            }
        }
    }
}

pub fn parse_instructions(input: &str) -> Result<Vec<Instruction>, &str> {
    input
        .trim()
        .lines()
        .map(|l| Instruction::try_from(l))
        .collect::<Result<Vec<_>, _>>()
}

pub fn circuit_map<'a>(instructions: Vec<Instruction<'a>>) -> HashMap<&'a str, SignalSource<'a>> {
    instructions
        .into_iter()
        .map(|i| (i.wire, i.source))
        .collect::<HashMap<&str, SignalSource<'a>>>()
}

pub fn calculate_signals_until<'a, F>(
    circuit_map: &mut HashMap<&'a str, SignalSource<'a>>,
    condition: F,
) -> HashMap<&'a str, SignalSource<'a>>
where
    F: Fn(&mut HashMap<&'a str, SignalSource<'a>>) -> bool,
{
    let mut resolved: HashMap<&str, SignalSource> = HashMap::new();
    loop {
        for (k, v) in circuit_map.iter_mut() {
            match v {
                SignalSource::FromValue(_) => {
                    resolved.insert(k, *v);
                }
                SignalSource::FromWire { wire_id } => {
                    if let Some(SignalSource::FromValue(val)) = resolved.get(wire_id) {
                        *v = SignalSource::FromValue(*val);
                        resolved.insert(k, *v);
                    }
                }
                SignalSource::FromValueAndGate { value, wire_id } => {
                    if let Some(SignalSource::FromValue(val)) = resolved.get(wire_id) {
                        *v = SignalSource::FromValue((value).bitand(*val));
                        resolved.insert(k, *v);
                    }
                }
                SignalSource::FromAndGate { wire_id1, wire_id2 } => {
                    if let Some(SignalSource::FromValue(val)) = resolved.get(wire_id1) {
                        *v = SignalSource::FromValueAndGate {
                            value: *val,
                            wire_id: &wire_id2,
                        };
                    } else if let Some(SignalSource::FromValue(val)) = resolved.get(wire_id2) {
                        *v = SignalSource::FromValueAndGate {
                            value: *val,
                            wire_id: &wire_id1,
                        };
                    }
                }
                SignalSource::FromValueOrGate { value, wire_id } => {
                    if let Some(SignalSource::FromValue(val)) = resolved.get(wire_id) {
                        *v = SignalSource::FromValue((*value).bitor(*val));
                        resolved.insert(k, *v);
                    }
                }
                SignalSource::FromOrGate { wire_id1, wire_id2 } => {
                    if let Some(SignalSource::FromValue(val)) = resolved.get(wire_id1) {
                        *v = SignalSource::FromValueOrGate {
                            value: *val,
                            wire_id: &wire_id2,
                        };
                    } else if let Some(SignalSource::FromValue(val)) = resolved.get(wire_id2) {
                        *v = SignalSource::FromValueOrGate {
                            value: *val,
                            wire_id: &wire_id1,
                        };
                    }
                }
                SignalSource::FromLShiftGate { wire_id, value } => {
                    if let Some(SignalSource::FromValue(val)) = resolved.get(wire_id) {
                        *v = SignalSource::FromValue(val << *value);
                        resolved.insert(k, *v);
                    }
                }
                SignalSource::FromRShiftGate { wire_id, value } => {
                    if let Some(SignalSource::FromValue(val)) = resolved.get(wire_id) {
                        *v = SignalSource::FromValue(val >> *value);
                        resolved.insert(k, *v);
                    }
                }
                SignalSource::FromNotGate { wire_id } => {
                    if let Some(SignalSource::FromValue(val)) = resolved.get(wire_id) {
                        *v = SignalSource::FromValue(!val);
                        resolved.insert(k, *v);
                    }
                }
            }
        }

        if condition(circuit_map) {
            break;
        }
    }
    resolved
}

pub fn a_found(circuit_map: &mut HashMap<&str, SignalSource<'_>>) -> bool {
    matches!(circuit_map.get("a"), Some(SignalSource::FromValue(_)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_for_instruction() {
        assert_eq!(
            Instruction::try_from("123 -> x"),
            Ok(Instruction {
                source: SignalSource::FromValue(123),
                wire: "x"
            })
        );
        assert_eq!(
            Instruction::try_from("x AND y -> d"),
            Ok(Instruction {
                source: SignalSource::FromAndGate {
                    wire_id1: "x",
                    wire_id2: "y"
                },
                wire: "d"
            })
        );
        assert_eq!(
            Instruction::try_from("x LSHIFT 2 -> f"),
            Ok(Instruction {
                source: SignalSource::FromLShiftGate {
                    wire_id: "x",
                    value: 2
                },
                wire: "f"
            })
        );
        assert_eq!(
            Instruction::try_from("NOT y -> i"),
            Ok(Instruction {
                source: SignalSource::FromNotGate { wire_id: "y" },
                wire: "i"
            })
        );
        assert_eq!(
            Instruction::try_from("lx -> a"),
            Ok(Instruction {
                source: SignalSource::FromWire { wire_id: "lx" },
                wire: "a"
            })
        );
        assert_eq!(
            Instruction::try_from("1 AND bh -> bi"),
            Ok(Instruction {
                source: SignalSource::FromValueAndGate {
                    value: 1,
                    wire_id: "bh"
                },
                wire: "bi"
            })
        );
    }
}
