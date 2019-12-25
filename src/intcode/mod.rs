mod interpreter;
mod memory;
mod parser;

pub use interpreter::Interpreter;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_parse_op_sum() {
//         let prg = Interpreter::from_bytecode(&vec![1, 4, 5, 6, 2, 243, 0]);
//         let op = prg.parse_op(0);

//         assert_eq!(op, Op::Sum(4, 5, 6));
//     }

//     #[test]
//     fn test_parse_op_multiply() {
//         let prg = IntcodeInterpreter::new(vec![2, 4, 5, 6, 2, 243, 0]);
//         let op = prg.parse_op(0);

//         assert_eq!(op, Op::Multiply(4, 5, 6));
//     }

//     #[test]
//     fn test_parse_op_halt() {
//         let prg = IntcodeInterpreter::new(vec![99, 4, 5, 6, 2, 243, 0]);
//         let op = prg.parse_op(0);

//         assert_eq!(op, Op::Halt);
//     }

//     #[test]
//     fn test_execute() {
//         let programs: Vec<(Vec<i32>, Vec<i32>)> = vec![
//             (
//                 vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
//                 vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
//             ),
//             (vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]),
//             (vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]),
//             (vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]),
//             (
//                 vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
//                 vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
//             ),
//         ];

//         for (starting_memory, end_memory) in programs {
//             let mut program = IntcodeInterpreter::new(starting_memory);
//             program.execute();
//             assert_eq!(program.memory, end_memory)
//         }
//     }
// }
