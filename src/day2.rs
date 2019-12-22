use crate::intcode::IntcodeInterpreter;

pub fn run(src: &Vec<i32>) {
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

            let mut program = IntcodeInterpreter::new(test_memory);
            let response = program.execute();
            if response == 19690720 {
                println!("Found answer {} with noun {} and verb {}", response, i, j);
                break 'outer;
            }
        }
    }
}
