#[derive(Debug, PartialEq)]
enum Op {
    Sum(usize, usize, usize),
    Multiply(usize, usize, usize),
    Halt,
}

pub struct IntcodeInterpreter {
    memory: Vec<i32>,
    head: usize,
}

impl IntcodeInterpreter {
    pub fn new(source: Vec<i32>) -> IntcodeInterpreter {
        IntcodeInterpreter {
            memory: source,
            head: 0,
        }
    }

    pub fn execute(&mut self) {
        loop {
            let op = self.parse_op_at_head();

            if !self.execute_op(&op) {
                break;
            }
        }
    }

    pub fn get(&self, at: usize) -> i32 {
        self.memory[at]
    }

    fn execute_op(&mut self, op: &Op) -> bool {
        match op {
            Op::Sum(loc_1, loc_2, loc_result) => {
                self.memory[*loc_result] = self.memory[*loc_1] + self.memory[*loc_2];
                true
            }

            Op::Multiply(loc_1, loc_2, loc_result) => {
                self.memory[*loc_result] = self.memory[*loc_1] * self.memory[*loc_2];
                true
            }

            Op::Halt => false,
        }
    }

    fn parse_op_at_head(&mut self) -> Op {
        let op_code = self.memory[self.head];
        self.head += 1;

        match op_code {
            99 => Op::Halt,
            1 => Op::Sum(
                self.read_pointer_at_head(),
                self.read_pointer_at_head(),
                self.read_pointer_at_head(),
            ),
            2 => Op::Multiply(
                self.read_pointer_at_head(),
                self.read_pointer_at_head(),
                self.read_pointer_at_head(),
            ),

            _ => panic!("Unknown op code"),
        }
    }

    fn read_pointer_at_head(&mut self) -> usize {
        let pos = self.memory[self.head];
        self.head += 1;

        if pos < 0 || (pos as usize) >= self.memory.len() {
            panic!("Invalid memory referenced");
        }

        return pos as usize;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let mut prg = IntcodeInterpreter::new(vec![123, 243, 0]);
        let op = Op::Sum(0, 1, 2);

        prg.execute_op(&op);
        assert_eq!(prg.memory, vec![123, 243, 366]);
    }

    #[test]
    fn test_mul() {
        let mut prg = IntcodeInterpreter::new(vec![2, 243, 0]);
        let op = Op::Multiply(0, 1, 2);

        prg.execute_op(&op);
        assert_eq!(prg.memory, vec![2, 243, 486]);
    }

    #[test]
    fn test_halt() {
        let mut prg = IntcodeInterpreter::new(vec![2, 243, 0]);
        let op = Op::Halt;

        prg.execute_op(&op);
        assert_eq!(prg.memory, vec![2, 243, 0]);
    }

    #[test]
    fn test_parse_op_sum() {
        let mut prg = IntcodeInterpreter::new(vec![1, 4, 5, 6, 2, 243, 0]);
        let op = prg.parse_op_at_head();

        assert_eq!(op, Op::Sum(4, 5, 6));
    }

    #[test]
    fn test_parse_op_multiply() {
        let mut prg = IntcodeInterpreter::new(vec![2, 4, 5, 6, 2, 243, 0]);
        let op = prg.parse_op_at_head();

        assert_eq!(op, Op::Multiply(4, 5, 6));
    }

    #[test]
    fn test_parse_op_halt() {
        let mut prg = IntcodeInterpreter::new(vec![99, 4, 5, 6, 2, 243, 0]);
        let op = prg.parse_op_at_head();

        assert_eq!(op, Op::Halt);
    }

    #[test]
    fn test_execute() {
        let programs: Vec<(Vec<i32>, Vec<i32>)> = vec![
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
            let mut program = IntcodeInterpreter::new(starting_memory);
            program.execute();
            assert_eq!(program.memory, end_memory)
        }
    }
}
