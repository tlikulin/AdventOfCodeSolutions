use std::collections::HashMap;

#[derive(Debug)]
enum Operand {
    Variable(String),
    Literal(u16),
}

impl Operand {
    fn new(oper: &str) -> Operand {
        if oper.as_bytes()[0].is_ascii_alphabetic() {
            Operand::Variable(oper.to_string())
        } else {
            Operand::Literal(oper.parse().unwrap())
        }
    }

    fn is_known(&self, map: &HashMap<String, u16>) -> bool {
        match self {
            Operand::Variable(name) => map.contains_key(name),
            Operand::Literal(_) => true,
        }
    }

    fn get_value(&self, map: &HashMap<String, u16>) -> u16 {
        match self {
            Operand::Variable(name) => *map.get(name).unwrap(),
            &Operand::Literal(value) => value,
        }
    }
}

#[derive(Debug)]
enum Operation {
    And(Operand, Operand),
    Or(Operand, Operand),
    Lshift(Operand, Operand),
    Rshift(Operand, Operand),
    Not(Operand),
    Load(Operand),
}

#[derive(Debug)]
struct Instruction {
    op: Operation,
    to: String,
}

impl Instruction {
    fn new(line: &str) -> Instruction {
        let mut iter = line.split_ascii_whitespace();
        let first = iter.next().unwrap();

        if first == "NOT" {
            let operand = Operand::new(iter.next().unwrap());

            iter.next(); // consume "->"
            let to = iter.next().unwrap().to_string();

            Instruction {
                op: Operation::Not(operand),
                to,
            }
        } else {
            let operand1 = Operand::new(first);

            let operation = iter.next().unwrap();
            if operation == "->" {
                let to = iter.next().unwrap().to_string();

                return Instruction {
                    op: Operation::Load(operand1),
                    to,
                };
            }

            let operand2 = Operand::new(iter.next().unwrap());

            iter.next(); // consume "->"
            let to = iter.next().unwrap().to_string();

            let op = match operation {
                "AND" => Operation::And(operand1, operand2),
                "OR" => Operation::Or(operand1, operand2),
                "LSHIFT" => Operation::Lshift(operand1, operand2),
                "RSHIFT" => Operation::Rshift(operand1, operand2),
                _ => panic!(),
            };

            Instruction { op, to }
        }
    }

    fn can_execute(&self, map: &HashMap<String, u16>) -> bool {
        match &self.op {
            Operation::And(operand1, operand2)
            | Operation::Or(operand1, operand2)
            | Operation::Lshift(operand1, operand2)
            | Operation::Rshift(operand1, operand2) => {
                operand1.is_known(map) && operand2.is_known(map)
            }
            Operation::Not(operand) | Operation::Load(operand) => operand.is_known(map),
        }
    }

    fn execute(&self, map: &mut HashMap<String, u16>) {
        let value = match &self.op {
            Operation::And(operand1, operand2) => operand1.get_value(map) & operand2.get_value(map),
            Operation::Or(operand1, operand2) => operand1.get_value(map) | operand2.get_value(map),
            Operation::Lshift(operand1, operand2) => {
                operand1.get_value(map) << operand2.get_value(map)
            }
            Operation::Rshift(operand1, operand2) => {
                operand1.get_value(map) >> operand2.get_value(map)
            }
            Operation::Not(operand) => !operand.get_value(map),
            Operation::Load(operand) => operand.get_value(map),
        };

        map.insert(self.to.clone(), value);
    }
}

// manually change the line that inits b in the file using the output from part 1
fn main() {
    let input = std::fs::read_to_string("input_part2.txt").unwrap();
    let mut instructions: Vec<_> = input.lines().map(Instruction::new).collect();
    let mut map: HashMap<String, u16> = HashMap::new();

    let mut count = 0;
    while !map.contains_key("a") {
        instructions = instructions
            .into_iter()
            .filter_map(|inst| {
                if inst.can_execute(&map) {
                    inst.execute(&mut map);
                    None
                } else {
                    Some(inst)
                }
            })
            .collect();

        count += 1;
    }

    println!("{} after {count} iterations", map.get("a").unwrap());
}
