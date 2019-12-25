use super::memory::ReadOnlyMemoryManager;
use std::rc::Rc;

type PositionMode = usize;
type ImmediateMode = isize;

#[derive(Debug)]
pub enum Op {
    Sum(Param, Param, PositionMode),
    Multiply(Param, Param, PositionMode),
    Input(PositionMode),
    Output(PositionMode),
    Halt,
}

#[derive(Debug)]
pub enum Param {
    PositionMode(PositionMode),
    ImmediateMode(ImmediateMode),
}

pub struct Parser {
    memory: Rc<dyn ReadOnlyMemoryManager>,
}

impl Parser {
    pub fn new(memory: Rc<dyn ReadOnlyMemoryManager>) -> Parser {
        Parser { memory }
    }

    pub fn parse_op(&self, at: usize) -> Op {
        let op_code = self.memory.read(at) % 100;
        let param_modes = (self.memory.read(at) / 100) as usize;

        match op_code {
            99 => Op::Halt,

            1 => Op::Sum(
                self.read_parameter(at + 1, param_modes % 10),
                self.read_parameter(at + 2, param_modes / 10 % 10),
                self.memory.read_address(at + 3),
            ),

            2 => Op::Multiply(
                self.read_parameter(at + 1, param_modes % 10),
                self.read_parameter(at + 2, param_modes / 10 % 10),
                self.memory.read_address(at + 3),
            ),

            3 => Op::Input(self.memory.read_address(at + 1)),

            4 => Op::Output(self.memory.read_address(at + 1)),

            x => panic!(format!("Unknown op code {}", x)),
        }
    }

    fn read_parameter(&self, at: usize, flag: usize) -> Param {
        if flag == 1 {
            Param::ImmediateMode(self.memory.read(at))
        } else {
            Param::PositionMode(self.memory.read_address(at))
        }
    }
}

pub fn parse_bytecode_string(src: &str) -> Vec<isize> {
    src.split_whitespace()
        .flat_map(|line| {
            line.split(',').flat_map(|val| {
                let word = val.parse::<isize>();
                match word {
                    Ok(val) => vec![val],
                    Err(_) => {
                        println!("[{}] did not produce a good value", val);
                        vec![]
                    }
                }
            })
        })
        .collect()
}
