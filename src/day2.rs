/*
An Intcode program is a list of integers separated by commas (like 1,0,0,3,99).
To run one, start by looking at the first integer (called position 0).
Here, you will find an opcode - either 1, 2, or 99.
The opcode indicates what to do; for example, 99 means that the program is finished and should
immediately halt. Encountering an unknown opcode means something went wrong.

Opcode 1 adds together numbers read from two positions and stores the result in a third position.
The three integers immediately after the opcode tell you these three positions -
the first two indicate the positions from which you should read the input values,
and the third indicates the position at which the output should be stored.

Opcode 2 works exactly like opcode 1, except it multiplies the two inputs instead of adding them.
Again, the three integers after the opcode indicate where the inputs and outputs are, not their values

Once you're done processing an opcode, move to the next one by stepping forward 4 positions.

*/
#[allow(dead_code)]
pub mod part1 {
    const OPCODE_ADD:u32 = 1;
    const OPCODE_MUL:u32 = 2;
    const OPCODE_STOP:u32 = 99;

    pub fn run() {
        let mut intcode: Vec<u32> = vec![
            1,0,0,3,
            1,1,2,3,
            1,3,4,3,
            1,5,0,3,
            2,13,1,19,
            1,19,10,23,
            1,23,13,27,
            1,6,27,31,
            1,9,31,35,
            2,10,35,39,
            1,39,6,43,
            1,6,43,47,
            2,13,47,51,
            1,51,6,55,
            2,6,55,59,
            2,59,6,63,
            2,63,13,67,
            1,5,67,71,
            2,9,71,75,
            1,5,75,79,
            1,5,79,83,
            1,83,6,87,
            1,87,6,91,
            1,91,5,95,
            2,10,95,99,
            1,5,99,103,
            1,10,103,107,
            1,107,9,111,
            2,111,10,115,
            1,115,9,119,
            1,13,119,123,
            1,123,9,127,
            1,5,127,131,
            2,13,131,135,
            1,9,135,139,
            1,2,139,143,
            1,13,143,0,
            99,
            2,0,14,0];
        let mut pc = 0;
        let mut instruction_line = 0;
        let mut opcode:u32 = intcode[pc];
        while opcode != OPCODE_STOP {
            //TODO: check for out of range opcode-reading
            instruction_line = instruction_line + 1;
            print!("{} ",instruction_line);
            if opcode == OPCODE_ADD {
                let operand1:usize = intcode[pc+1] as usize;
                let operand2:usize = intcode[pc+2]  as usize;
                let dest:usize = intcode[pc+3]  as usize;
                intcode[dest] = intcode[operand1] + intcode[operand2];
                println!(" add {}({}) and {}({}) to {} ", intcode[operand1], operand1, intcode[operand2], operand2, dest);

            }
            else if opcode == OPCODE_MUL {
                let operand1:usize = intcode[pc+1]  as usize;
                let operand2:usize = intcode[pc+2] as usize;
                let dest:usize = intcode[pc+3] as usize;
                intcode[dest] = intcode[operand1] * intcode[operand2];
                println!(" mul {}({}) and {}({}) to {} ", intcode[operand1], operand1, intcode[operand2], operand2, dest);

            }
            pc = pc + 4;
            opcode = intcode[pc];

        }

        println!("{:?}", intcode);
        //1450628 is to low
    }
}