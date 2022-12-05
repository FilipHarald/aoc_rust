use std::collections::HashMap;
use std::collections::VecDeque;
// TODO: compare performance difference by implementing with Vector instead.

#[derive(Debug)]
enum GateType {
    And,
    Or,
    Lshift,
    Rshift,
    Not,
    NoGate,
}

#[derive(Debug)]
struct InstructionsA<'a> {
    gate_type: Option<GateType>,
    arg_1: &'a str,
    arg_2: Option<&'a str>,
    output_name: &'a str,
}

fn parse_instructions(line: &str) -> InstructionsA {
    let mut gate_type = None;
    let arg_1;
    let mut arg_2 = None;
    let parts: Vec<&str> = line.split("->").collect();
    let output_name = parts[1].trim_start();

    let input_parts: Vec<&str> = parts[0].split(" ").collect();
    if input_parts[0] == "NOT" {
        gate_type = Some(GateType::Not);
        arg_1 = input_parts[1];
    } else if input_parts.len() == 4 {
        arg_1 = input_parts[0];
        arg_2 = Some(input_parts[2]);
        match input_parts[1] {
            "AND" => gate_type = Some(GateType::And),
            "OR" => gate_type = Some(GateType::Or),
            "LSHIFT" => gate_type = Some(GateType::Lshift),
            "RSHIFT" => gate_type = Some(GateType::Rshift),
            _ => println!("Error in input '{}' is not a gate type.", input_parts[1]),
        }
    } else {
        arg_1 = input_parts[0];
        gate_type = Some(GateType::NoGate);
    }
    return InstructionsA {
        gate_type,
        arg_1,
        arg_2,
        output_name,
    };
}

fn get_value(input: &str, wires: &HashMap<String, u16>) -> Option<u16> {
    match input.parse::<u16>() {
        Ok(nbr) => Some(nbr),
        Err(_) => {
            if wires.contains_key(input) {
                return Some(wires.get(input).unwrap().to_owned());
            }
            return None;
        }
    }
}

pub fn solve_a(input: &str) -> String {
    let mut instructions: VecDeque<InstructionsA> = VecDeque::new();
    for l in input.lines() {
        let ins = parse_instructions(l);
        instructions.push_front(ins);
    }
    let mut wires: HashMap<String, u16> = HashMap::new();
    while let Some(ins) = instructions.pop_back() {
        let mut res: Option<u16> = None;
        match ins.gate_type.as_ref().unwrap() {
            GateType::And => {
                let value_1 = get_value(ins.arg_1, &wires);
                let value_2 = get_value(ins.arg_2.unwrap(), &wires);
                if value_1.is_some() && value_2.is_some() {
                    res = Some(value_1.unwrap() & value_2.unwrap());
                }
            }
            GateType::Or => {
                let value_1 = get_value(ins.arg_1, &wires);
                let value_2 = get_value(ins.arg_2.unwrap(), &wires);
                if value_1.is_some() && value_2.is_some() {
                    res = Some(value_1.unwrap() | value_2.unwrap());
                }
            }
            GateType::Lshift => {
                let value_1 = get_value(ins.arg_1, &wires);
                let value_2 = get_value(ins.arg_2.unwrap(), &wires);
                if value_1.is_some() && value_2.is_some() {
                    res = Some(value_1.unwrap() << value_2.unwrap());
                }
            }
            GateType::Rshift => {
                let value_1 = get_value(ins.arg_1, &wires);
                let value_2 = get_value(ins.arg_2.unwrap(), &wires);
                if value_1.is_some() && value_2.is_some() {
                    res = Some(value_1.unwrap() >> value_2.unwrap());
                }
            }
            GateType::Not => {
                let value_1 = get_value(ins.arg_1, &wires);
                if value_1.is_some() {
                    res = Some(!value_1.unwrap());
                }
            }
            GateType::NoGate => {
                let value_1 = get_value(ins.arg_1, &wires);
                if value_1.is_some() {
                    res = Some(value_1.unwrap());
                }
            }
        }
        match res {
            Some(value)=> {
                wires.insert(ins.output_name.to_string(), value);
            }
            None=> {
                instructions.push_front(ins);
            }
        }
    }
    return wires.get("a").unwrap().to_string();
}

pub fn solve_b(input: &str) -> String {
    // Just changed the input. Did I make it too easy?
    return solve_a(input);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_1() {
        let input = "123 -> a";
        let result = "123";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_2() {
        let input = "321 -> b
123 -> a";
        let result = "123";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_3() {
        let input = "123 -> x
456 -> y
x AND y -> a";
        let result = "72";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_4() {
        let input = "123 -> x
456 -> y
x OR y -> a";
        let result = "507";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_5() {
        let input = "123 -> x
x LSHIFT 2 -> a";
        let result = "492";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_6() {
        let input = "456 -> y
y RSHIFT 2 -> a";
        let result = "114";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_7() {
        let input = "123 -> x
NOT x -> a";
        let result = "65412";
        assert_eq!(solve_a(input), result);
    }
    #[test]
    fn a_8() {
        let input = "456 -> y
NOT y -> a";
        let result = "65079";
        assert_eq!(solve_a(input), result);
    }
}
