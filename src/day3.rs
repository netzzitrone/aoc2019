#[allow(dead_code)]
pub mod part1 {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq, Hash)]
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
        /* println!("-------------");
         println!("{} {}", x, y);
         println!("{} steps to {}", length, instruction);
         println!("-------------");
         println!("");*/
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
                        Some(_x) => {

                            if *state.unwrap() != wire {
                                fields.insert(point, 9 );
                            }
                        }
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
                        Some(_x) => {
                            if *state.unwrap() != wire {
                                fields.insert(point, 9 );
                            }
                        }
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
                        Some(_x) => {
                            if *state.unwrap() != wire {
                                fields.insert(point, 9 );
                            }
                        }
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
                        Some(_x) => {
                            if *state.unwrap() != wire {
                                fields.insert(point, 9 );
                            }
                        }
                        None => {
                            fields.insert(point, wire );
                        },
                    }
                }
            },
            _ => unreachable!(), //wtf? https://jijnasu.in/rust-non-exhaustive-patterns-_-not-covered-pattern-_-not-covered/
        }
        (newx, newy, fields)
    }

    pub fn run() {
        let f = File::open("input/input3.csv").unwrap();
        let file = BufReader::new(&f);

        // let mut newCellState: CellState;
        let mut fields: Field = HashMap::new();
        for (_num, line) in file.lines().enumerate() {
            let mut x = 0;
            let mut y = 0;
            let instruction_line  = line.unwrap();
            let instructions = instruction_line.split(',');
            for instruction in instructions.collect::<Vec<&str>>() {
                let new_coords = draw_line(instruction, x, y, fields, _num);
                let (_x, _y, _fields) = new_coords;
                x = _x;
                y = _y;
                fields = _fields;
                //  println!("{} {}", x,y);
            }
        }
        let intersect:usize = 9;

        let mut mdist:i16 = 0;
        for (key, val) in fields.iter() {
            if val == &intersect {
                let abs = key.x.abs()+key.y.abs();
                if mdist == 0 ||  abs < mdist {
                    mdist = abs;
                }

                print!("x{}y{},", key.x, key.y);
            }
        }
        println!("minimal distance {}", mdist);
        //439 is to low
    }
}

pub mod part2 {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq, Hash)]
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



    fn draw_line (instruction: &str, x:i16, y:i16, fields:Field, wire:usize, wire_count:i32, intersect_length: bool, intersect_x:i16, intersect_y:i16, wire_sum:i32 ) -> (i16, i16, Field, i32, i32) {
        let mut instruction: String = instruction.into();
        let mut fields= fields;
        let mut new_wire_sum = wire_sum;
        let length = instruction.split_off(1).parse::<i16>().unwrap();
        let mut newx = x;
        let mut newy = y;
        let mut new_wire_count = wire_count;

        match instruction.as_str() {
            "R" => {
                for _i in 0..length {
                    newx = newx + 1;
                    new_wire_count = new_wire_count + 1;
                    let point:Point = Point {x:newx, y:newy};
                    let state = fields.get(&point);
                    match state {
                        Some(_x) => {

                            if *state.unwrap() != wire {
                                fields.insert(point, 9 );
                            }
                        }
                        None => {
                            fields.insert(point, wire );
                        },
                    }
                    if intersect_length == true && newx==intersect_x && newy == intersect_y {
                        println!("Wire {}, Length {}", wire, new_wire_count);
                        new_wire_sum =new_wire_sum + new_wire_count;
                    }
                }
            },
            "L" => {
                if wire == 0 && intersect_length == true {
                    //println!("{} {} {}---- ", newx, newy, wire_count);
                }
                for _i in 0..length {
                    newx = newx - 1;
                    new_wire_count = new_wire_count + 1;
                    let point:Point = Point {x:newx, y:newy};
                    let state = fields.get(&point);
                    match state {
                        Some(_x) => {
                            if *state.unwrap() != wire {
                                fields.insert(point, 9 );
                            }
                        }
                        None => {
                            fields.insert(point, wire);
                        },
                    }
                    if intersect_length == true && newx==intersect_x && newy == intersect_y {
                        println!("2Wire {}, Length {}", wire, new_wire_count);
                        new_wire_sum =new_wire_sum + new_wire_count;
                    }
                }
            },
            "U" => {
                for _i in 0..length {
                    newy = newy - 1;
                    new_wire_count = new_wire_count + 1;
                    let point:Point = Point {x:newx, y:newy};
                    let state = fields.get(&point);
                    match state {
                        Some(_x) => {
                            if *state.unwrap() != wire {
                                fields.insert(point, 9 );
                            }
                        }
                        None => {
                            fields.insert(point, wire );
                        },
                    }
                    if intersect_length == true && newx==intersect_x && newy == intersect_y {
                        println!("Wire {}, Length {}", wire, new_wire_count);
                        new_wire_sum =new_wire_sum + new_wire_count;
                    }
                }
            },
            "D" => {
                for _i in 0..length {
                    newy = newy + 1;
                    new_wire_count = new_wire_count + 1;
                    let point:Point = Point {x:newx, y:newy};
                    let state = fields.get(&point);
                    match state {
                        Some(_x) => {
                            if *state.unwrap() != wire {
                                fields.insert(point, 9 );
                            }
                        }
                        None => {
                            fields.insert(point, wire );
                        },
                    }
                    if intersect_length == true && newx==intersect_x && newy == intersect_y {
                        println!("4Wire {}, Length {}", wire, new_wire_count);
                        new_wire_sum =new_wire_sum + new_wire_count;
                    }
                }
            },
            _ => unreachable!(), //wtf? https://jijnasu.in/rust-non-exhaustive-patterns-_-not-covered-pattern-_-not-covered/
        }

        // println!("{} {}", newx, newy);
        (newx, newy, fields, new_wire_count, new_wire_sum)
    }

    pub fn run() {
        let f = File::open("input/input3.csv").unwrap();
        let file = BufReader::new(&f);

        let mut fields: Field = HashMap::new();
        for (_num, line) in file.lines().enumerate() {
            let mut x = 0;
            let mut y = 0;
            let instruction_line  = line.unwrap();
            let instructions = instruction_line.split(',');
            let mut wire_count = 0;
            for instruction in instructions.collect::<Vec<&str>>() {
                let new_coords = draw_line(instruction, x, y, fields, _num, wire_count, false, 0, 0, 0);
                let (_x, _y, _fields, _wire_count, _new_wire_sum) = new_coords;
                x = _x;
                y = _y;
                fields = _fields;
                wire_count = _wire_count;

            }
        }
        let intersect:usize = 9;

        let mut mdist:i16 = 0;
        for (key, val) in fields.iter() {
            if val == &intersect {
                let abs = key.x.abs()+key.y.abs();
                if mdist == 0 ||  abs < mdist {
                    mdist = abs;
                }
                println!("key: {} {} val: {}", key.x, key.y, val);

                let f = File::open("input/input3.csv").unwrap();
                let file = BufReader::new(&f);
                let mut fields: Field = HashMap::new();
                let mut new_wire_sum = 0;
                for (_num, line) in file.lines().enumerate() {
                    let mut x = 0;
                    let mut y = 0;
                    let instruction_line  = line.unwrap();
                    let instructions = instruction_line.split(',');
                    let mut wire_count = 0;

                    for instruction in instructions.collect::<Vec<&str>>() {
                        let new_coords = draw_line(instruction, x, y, fields, _num, wire_count, true, key.x, key.y, new_wire_sum);
                        let (_x, _y, _fields, _wire_count, _new_wire_sum) = new_coords;
                        x = _x;
                        y = _y;
                        fields = _fields;
                        new_wire_sum = _new_wire_sum;
                        wire_count = _wire_count;
                    }
                }

                println!("Wire-Sum: {}",new_wire_sum );
                println!("-----------------");

            }
        }
        println!("minimal distance {}", mdist);
        //439 is to low
    }
}
