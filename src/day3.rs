#[allow(dead_code)]
pub mod part1 {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;

    struct Point {
        x: u8,
        y: u8
    }


    fn draw_line (instruction: &str, x:i16, y:i16 ) -> (i16, i16) {
        let mut instruction: String = instruction.into();
        let length = instruction.split_off(1).parse::<i16>().unwrap();
       // let direction = instruction.remove(0);
        //let length = instruction;
        println!("-------------");
        println!("{} {}", x, y);
        println!("{} steps to {}", length, instruction);
        //direction = direction.to_string();
      // println!("{:?}", instruction);i
        // direction = direction as String;
        let mut newx = x;
        let mut newy = y;
        match instruction.as_str() {
            "R" => newx = newx + length,
            "L" => newx = newx - length,
            "U" => newy = newy - length,
            "D" => newy = newy + length,
            _ => println!("blahh blahhh"), //wtf?
        }
        println!("{} {}", newx, newy);
        (newx, newy)
    }

    pub fn run() {
        let f = File::open("input/input3_small.csv").unwrap();
        let file = BufReader::new(&f);
        let mut x = 0;
        let mut y = 0;
        for (_num, line) in file.lines().enumerate() {
            let instruction_line  = line.unwrap();
            let instructions = instruction_line.split(',');
            for (instruction) in instructions.collect::<Vec<&str>>() {
                let newCoords = draw_line(instruction, x, y);
                let (_x, _y) = newCoords;
                x = _x;
                y = _y;
                println!("{} {}", x,y);
            }
        }

    }


}


