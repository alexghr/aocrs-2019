use std::io::prelude::*;
use std::io::{stdin, stdout};
use std::rc::Rc;

use super::memory::{MemoryManager, MutableMemoryManager, ReadOnlyMemoryManager};
use super::parser::{parse_bytecode_string, Op, Param, Parser};

pub struct Interpreter {
    memory: Rc<dyn MutableMemoryManager>,
    instruction_pointer: usize,
    parser: Parser,
}

impl Interpreter {
    pub fn from_string(src: &str) -> Interpreter {
        let init_memory = parse_bytecode_string(src);
        let memory = Rc::new(MemoryManager::new(&init_memory));
        let parser = Parser::new(Rc::clone(&memory) as Rc<dyn ReadOnlyMemoryManager>);

        Interpreter {
            instruction_pointer: 0,
            memory,
            parser,
        }
    }

    pub fn from_bytecode(src: &Vec<isize>) -> Interpreter {
        let memory = Rc::new(MemoryManager::new(src));
        let parser = Parser::new(Rc::clone(&memory) as Rc<dyn ReadOnlyMemoryManager>);

        Interpreter {
            instruction_pointer: 0,
            memory,
            parser,
        }
    }

    pub fn execute(&mut self) {
        loop {
            let op = self.parser.parse_op(self.instruction_pointer);
            match op {
                Op::Halt => break,

                Op::Sum(a, b, addr) => {
                    self.memory
                        .write(addr, self.read_parameter(a) + self.read_parameter(b));
                    self.instruction_pointer += 4;
                }

                Op::Multiply(a, b, addr) => {
                    self.memory
                        .write(addr, self.read_parameter(a) * self.read_parameter(b));
                    self.instruction_pointer += 4;
                }

                Op::Input(addr) => {
                    let mut val = String::new();
                    if stdin().read_line(&mut val).is_ok() {
                        if let Ok(val) = val.split_whitespace().next().unwrap().parse::<isize>() {
                            self.memory.write(addr, val)
                        }
                    }

                    self.instruction_pointer += 2;
                }

                Op::Output(address) => {
                    if writeln!(stdout(), "{}", self.memory.read(address)).is_ok() {
                        //
                    }

                    self.instruction_pointer += 2;
                }

                _ => panic!("Unknown operation"),
            }
        }
    }

    pub fn read(&self, at: usize) -> isize {
        self.memory.read(at)
    }

    fn read_parameter(&self, param: Param) -> isize {
        match param {
            Param::PositionMode(at) => self.memory.read(at),
            Param::ImmediateMode(val) => val,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_execute() {
        let programs: Vec<(Vec<isize>, Vec<isize>)> = vec![
            (vec![99], vec![99]),
            (vec![1, 5, 6, 0, 99, 3, 7], vec![10, 5, 6, 0, 99, 3, 7]),
            (
                vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
                vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            ),
            (vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]),
            (vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]),
            (vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]),
            (
                vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
                vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
            ),
        ];

        for (starting_memory, end_memory) in programs {
            let mut program = Interpreter::from_bytecode(&starting_memory);
            program.execute();
            assert_eq!(program.memory.dump(), end_memory)
        }
    }

    #[test]
    fn test_execute_2() {
        let mut prg = Interpreter::from_string("1002,4,3,4,33");
        prg.execute();
        assert_eq!(prg.memory.dump(), vec![1002, 4, 3, 4, 99]);
    }
}
