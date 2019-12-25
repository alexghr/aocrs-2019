use crate::intcode::Interpreter;

pub fn run(src: &Vec<isize>) {
    // Part 1
    // the challenge says we have to do this
    // intcode_src[1] = 12;
    // intcode_src[2] = 2;
    // let mut program = IntcodeInterpreter::new(intcode_src);
    // let response = program.execute();

    // println!("value at position 0: {}", response);

    'outer: for i in 0..100 {
        for j in 0..100 {
            let mut test_memory = src.clone();
            test_memory[1] = i;
            test_memory[2] = j;

            let mut program = Interpreter::from_bytecode(&test_memory);
            program.execute();
            if program.read(0) == 19690720 {
                println!("Found answer {} with noun {} and verb {}", 19690720, i, j);
                break 'outer;
            }
        }
    }
}
