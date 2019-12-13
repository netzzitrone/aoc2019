#[allow(dead_code)]
pub mod part1 {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;

    fn fuel (mass: u32) -> u32 {
        mass / 3 -2
    }

    pub fn run() {
        let f = File::open("input.tsv").unwrap();
        let file = BufReader::new(&f);
        let mut all_fuel = 0;
        for (_num, line) in file.lines().enumerate() {
            let col = line.unwrap();
            let mass = col.parse().unwrap();
            all_fuel = all_fuel + fuel(mass);
        }
        println!("Fuel {}", all_fuel);
    }

    #[test]
    fn fuel_test() {
        assert_eq!(fuel(6), 0);
        assert_eq!(fuel(12), 2);
        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(1969), 654);
        assert_eq!(fuel(100756), 33583);
    }
}
#[allow(dead_code)]
pub mod part2 {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;

    pub fn fuel (mass: u32) -> u32 {
        let divided = mass / 3;
        match divided.checked_sub(2) {
            Some(_x) => divided.checked_sub(2).unwrap(),
            None    => 0,
        }
    }

    pub fn run() {
        let f = File::open("input.tsv").unwrap();
        let file = BufReader::new(&f);
        let mut all_fuel = 0;
        for (_num, line) in file.lines().enumerate() {
            let col = line.unwrap();
            let mass = col.parse().unwrap();
            let mut used_fuel = fuel(mass);
            all_fuel = all_fuel + used_fuel;

            while used_fuel >= 6 {
                used_fuel = fuel(used_fuel);
                all_fuel = all_fuel + used_fuel;
            }
        }
        println!("Fuel {}",all_fuel);
    }

    #[test]
    fn fuel_test() {
        assert_eq!(fuel(3), 0);
        assert_eq!(fuel(6), 0);
        assert_eq!(fuel(12), 2);
        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(1969), 654);
        assert_eq!(fuel(100756), 33583);
    }
}


