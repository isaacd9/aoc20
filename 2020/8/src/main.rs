use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead, Error, Lines, StdinLock};

#[derive(Debug, Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn parse_program(lines: Lines<StdinLock>) -> Result<Vec<Instruction>, io::Error> {
    let line_re = Regex::new(r"(?P<opcode>\w+) (?P<operand>.*)").unwrap();

    let program = lines
        .map(|li| li.unwrap())
        .map(|li| {
            let caps = line_re.captures(&li).unwrap();
            match &caps["opcode"] {
                "nop" => Instruction::Nop(caps["operand"].parse().unwrap()),
                "acc" => Instruction::Acc(caps["operand"].parse().unwrap()),
                "jmp" => Instruction::Jmp(caps["operand"].parse().unwrap()),
                _ => panic!("unexpected operand: {}", &caps["opcode"]),
            }
        })
        .collect();

    Ok(program)
}

fn execute_program(program: &Vec<Instruction>) -> Result<i32, String> {
    let mut accumulator = 0;
    let mut pc: i32 = 0;
    let mut total = 0;

    let mut order: Vec<usize> = vec![0; program.len()];

    use crate::Instruction::*;

    while pc < program.len() as i32 {
        if order[pc as usize] != 0 {
            return Err(format!("infinite loop! pc: {}, acc: {}", pc, accumulator));
        }

        order[pc as usize] = total;
        total += 1;

        let cur_inst = &program[pc as usize];

        match cur_inst {
            Nop(_) => pc += 1,
            Acc(amt) => {
                accumulator += amt;
                pc += 1;
            }
            Jmp(num) => pc += *num,
        };
    }
    Ok(accumulator)
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut prog = parse_program(lines).unwrap();

    let result = execute_program(&prog);

    for idx in 0..prog.len() {
        let inst = &prog[idx];
        let tmp = inst.clone();
        let new_inst = match inst {
            Instruction::Nop(k) => Instruction::Jmp(*k),
            Instruction::Jmp(k) => Instruction::Nop(*k),
            Instruction::Acc(k) => Instruction::Acc(*k),
        };
        prog[idx] = new_inst;
        let result = execute_program(&prog);
        prog[idx] = tmp;
        if result.is_ok() {
            println!("{:?}", result);
        }
    }

    //println!("{:?}", result);
}
