use num_bigint::BigUint;
use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
enum Instruction {
    Hlf(bool),
    Tpl(bool),
    Inc(bool),
    Jmp(isize),
    Jie(bool, isize),
    Jio(bool, isize),
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, arg) = s.split_once(' ').unwrap();
        match name {
            "hlf" => Ok(Instruction::Hlf(arg == "a")),
            "tpl" => Ok(Instruction::Tpl(arg == "a")),
            "inc" => Ok(Instruction::Inc(arg == "a")),
            "jmp" => Ok(Instruction::Jmp(arg.parse()?)),
            "jie" => {
                let (reg, offset) = arg.split_once(", ").unwrap();
                Ok(Instruction::Jie(reg == "a", offset.parse()?))
            }
            "jio" => {
                let (reg, offset) = arg.split_once(", ").unwrap();
                Ok(Instruction::Jio(reg == "a", offset.parse()?))
            }
            _ => panic!("Unknown instruction"),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let instructions: Vec<Instruction> = input.lines().flat_map(FromStr::from_str).collect();

    let one = BigUint::from(1u8);
    let two = BigUint::from(2u8);
    let three = BigUint::from(3u8);
    let (mut reg_a, mut reg_b, mut pc) = (BigUint::ZERO, BigUint::ZERO, 0);

    while pc < instructions.len() {
        match instructions[pc] {
            Instruction::Hlf(is_a) => {
                if is_a {
                    reg_a /= &two;
                } else {
                    reg_b /= &two;
                }
                pc += 1;
            }
            Instruction::Tpl(is_a) => {
                if is_a {
                    reg_a *= &three;
                } else {
                    reg_b *= &three;
                }
                pc += 1;
            }
            Instruction::Inc(is_a) => {
                if is_a {
                    reg_a += &one;
                } else {
                    reg_b += &one;
                }
                pc += 1;
            }
            Instruction::Jmp(offset) => pc = pc.saturating_add_signed(offset),
            Instruction::Jie(is_a, offset) => {
                if (is_a && &reg_a % &two == BigUint::ZERO)
                    || (!is_a && &reg_b % &two == BigUint::ZERO)
                {
                    pc = pc.saturating_add_signed(offset);
                } else {
                    pc += 1;
                }
            }
            Instruction::Jio(is_a, offset) => {
                if (is_a && reg_a == one) || (!is_a && reg_b == one) {
                    pc = pc.saturating_add_signed(offset);
                } else {
                    pc += 1;
                }
            }
        }
    }

    println!("{reg_b}");
}
