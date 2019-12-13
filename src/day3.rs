#[allow(dead_code)]
pub mod part1 {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;
    use std::collections::HashMap;
    use std::cell::Cell;
    // use std::cell::Cell;
//    use crate::day3::part1::CellState::Empty;

    #[derive(PartialEq, Eq, Hash)]
    struct Point {
        x: i16,
        y: i16
    }


    #[derive(PartialEq, Eq, Hash)]
    enum CellState {
        Empty,
        Wire1,
        Wire2,
        Intersect
    }

    type Field = HashMap<Point, usize>;



    fn draw_line (instruction: &str, x:i16, y:i16, fields:Field, wire:usize) -> (i16, i16, Field) {
        let mut instruction: String = instruction.into();
        let mut fields= fields;

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
            "R" => {
                for _i in 0..length {
                    newx = newx + 1;
                    let point:Point = Point {x:newx, y:newy};
                    let state = fields.get(&point);
                    match state {
                        Some(_x) => {fields.insert(point, 9 );}
                        None => {
                            fields.insert(point, wire );
                        },
                    }
                }
            },
            "L" => {
                for _i in 0..length {
                    newx = newx - 1;
                    let point:Point = Point {x:newx, y:newy};
                    let state = fields.get(&point);
                    match state {
                        Some(_x) => {fields.insert(point, 9 );}
                        None => {
                            fields.insert(point, wire);
                        },
                    }
                }
            },
            "U" => {
                for _i in 0..length {
                    newy = newy - 1;
                    let point:Point = Point {x:newx, y:newy};
                    let state = fields.get(&point);
                    match state {
                        Some(_x) => {fields.insert(point, 9 );}
                        None => {
                            fields.insert(point, wire );
                        },
                    }
                }
            },
            "D" => {
                for _i in 0..length {
                    newy = newy + 1;
                    let point:Point = Point {x:newx, y:newy};
                    let state = fields.get(&point);
                    match state {
                        Some(_x) => {fields.insert(point, 9 );}
                        None => {
                            fields.insert(point, wire );
                        },
                    }
                }
            },
            _ => println!("blahh blahhh"), //wtf? https://jijnasu.in/rust-non-exhaustive-patterns-_-not-covered-pattern-_-not-covered/
        }
        println!("{} {}", newx, newy);
        (newx, newy, fields)
    }

    pub fn run() {
        let f = File::open("input/input3_small.csv").unwrap();
        let file = BufReader::new(&f);
        let mut x = 0;
        let mut y = 0;
       // let mut newCellState: CellState;
        let mut fields: Field = HashMap::new();
        for (_num, line) in file.lines().enumerate() {
            let instruction_line  = line.unwrap();
            let instructions = instruction_line.split(',');
            for instruction in instructions.collect::<Vec<&str>>() {

                let new_coords = draw_line(instruction, x, y, fields, _num);
                let (_x, _y, _fields) = new_coords;
                x = _x;
                y = _y;
                fields = _fields;
                println!("{} {}", x,y);
            }
        }
        let intersect:usize = 9;

        for (key, val) in fields.iter() {

            if val == &intersect {
                println!("key: {} {} val: {}", key.x, key.y, val);
            }
        }

    }


}


