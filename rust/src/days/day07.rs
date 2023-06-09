use std::collections::HashMap;

use regex::{Match, Regex};

use crate::{read_input, Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let sol1: u16 = part_one();
    let sol2: u16 = part_two();

    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(Debug, Hash, Clone)]
enum Operation {
    AND,
    OR,
    NOT,
    RSHIFT,
    LSHIFT,
    SET,
}

#[derive(Debug)]
struct Wire {
    name: String,
    value: Option<u16>,
    instructions: Vec<Instruction>,
}

impl Wire {
    fn new(name: &str) -> Self {
        Wire {
            name: name.to_string(),
            value: None,
            instructions: vec![],
        }
    }
}

impl PartialEq for Wire {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Wire {}

#[derive(Debug, Hash, Clone)]
struct Instruction {
    operation: Operation,
    operands: Vec<String>,
}

fn part_one() -> u16 {
    let mut circuit = build_circuit();
    get_wire_value("a", &mut circuit)
}

fn part_two() -> u16 {
    let mut circuit = build_circuit();
    let mut b = circuit.get_mut("b").expect("no b");
    b.value = Some(part_one());
    get_wire_value("a", &mut circuit)
}

fn get_wire_value(wire_name: &str, circuit: &mut HashMap<String, Wire>) -> u16 {
    let wire = circuit
        .get(wire_name)
        .expect("Could not find wire in circuit");

    if let Some(x) = wire.value {
        return x;
    }

    let instructions = wire.instructions.clone();

    let mut value: u16 = 0;
    for instruction in instructions.iter().rev() {
        value = match instruction.operation {
            Operation::AND => {
                let a = instruction
                    .operands
                    .get(0)
                    .expect("AND: No operand at index 0");

                let b = instruction
                    .operands
                    .get(1)
                    .expect("AND: No operand at index 1");

                parse_or_get_wire_value(a, circuit) & parse_or_get_wire_value(b, circuit)
            }
            Operation::OR => {
                let a = instruction
                    .operands
                    .get(0)
                    .expect("OR: No operand at index 0");

                let b = instruction
                    .operands
                    .get(1)
                    .expect("OR: No operand at index 1");

                parse_or_get_wire_value(a, circuit) | parse_or_get_wire_value(b, circuit)
            }
            Operation::NOT => {
                let a = instruction
                    .operands
                    .get(0)
                    .expect("NOT: No operand at index 0");

                !parse_or_get_wire_value(a, circuit)
            }
            Operation::RSHIFT => {
                let a = instruction
                    .operands
                    .get(0)
                    .expect("RSHIFT: No operand at index 0");

                let amt = instruction
                    .operands
                    .get(1)
                    .expect("RSHIFT: No operand at index 1")
                    .parse::<u16>()
                    .expect("Could not parse amount of bits to RSHIFT");

                parse_or_get_wire_value(a, circuit) >> amt
            }
            Operation::LSHIFT => {
                let a = instruction
                    .operands
                    .get(0)
                    .expect("LSHIFT: No operand at index 0");

                let amt = instruction
                    .operands
                    .get(1)
                    .expect("LSHIFT: No operand at index 1")
                    .parse::<u16>()
                    .expect("Could not parse amount of bits to RSHIFT");

                parse_or_get_wire_value(a, circuit) << amt
            }
            Operation::SET => {
                let a = instruction
                    .operands
                    .get(0)
                    .expect("NOT: No operand at index 0");

                parse_or_get_wire_value(a, circuit)
            }
        };
    }

    let mut wire = circuit
        .get_mut(wire_name)
        .expect("Could not find wire in circuit");

    wire.value = Some(value);
    value
}

fn parse_or_get_wire_value(s: &str, circuit: &mut HashMap<String, Wire>) -> u16 {
    match s.parse::<u16>() {
        Ok(x) => x,
        Err(_) => get_wire_value(s, circuit),
    }
}

fn build_circuit() -> HashMap<String, Wire> {
    let input_str = read_input(7, 1);
    let mut map: HashMap<String, Wire> = HashMap::new();
    let operation_re = Regex::new(r"[A-Z]+").expect("Err construction operation RegEx");
    let operands_re = Regex::new(r"[a-z0-9]+").expect("Err construction operands RegEx");

    for line in input_str.split('\n').filter(|x| !x.is_empty()) {
        let mut split = line.split(" -> ");
        let instruction_str = split
            .next()
            .expect("Err getting instruction_str from split");
        let dest_str = split.next().expect("Err getting dest_str from split");

        let operation_match = operation_re.find(instruction_str);
        let operands_matches: Vec<Match> = operands_re.find_iter(instruction_str).collect();

        let mut operation = Operation::SET;

        if let Some(m) = operation_match {
            operation = match m.as_str() {
                "AND" => Operation::AND,
                "OR" => Operation::OR,
                "NOT" => Operation::NOT,
                "RSHIFT" => Operation::RSHIFT,
                "LSHIFT" => Operation::LSHIFT,
                _ => Operation::SET,
            };
        };

        let operands = operands_matches
            .iter()
            .map(|m| m.as_str().to_string())
            .collect();

        let instruction = Instruction {
            operation,
            operands,
        };

        if let Some(wire) = map.get_mut(dest_str) {
            wire.instructions.push(instruction);
        } else {
            let mut wire = Wire::new(dest_str);
            wire.instructions.push(instruction);
            map.insert(dest_str.to_string(), wire);
        }
    }

    map
}
