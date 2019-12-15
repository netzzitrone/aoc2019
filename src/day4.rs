#[allow(dead_code)]
pub mod part1 {

    //https://stackoverflow.com/questions/41536479/splitting-an-integer-into-individual-digits
    fn int_to_digits(n: i32) -> Vec<i32> {
        fn x_inner(n: i32, xs: &mut Vec<i32>) {
            if n >= 10 {
                x_inner(n / 10, xs);
            }
            xs.push(n % 10);
        }
        let mut xs = Vec::new();
        x_inner(n, &mut xs);
        xs
    }

    fn check_password(password: i32) -> bool {
/*
        let pwd = password.to_string();
        let mut previous_digit = None;
        let mut seen_duplicate = false;
        for digit in pwd.chars() {
            if let Some(previous_digit) = previous_digit {
                if digit < previous_digit {
                    return false;
                }
                if digit == previous_digit {
                    seen_duplicate = true;
                }
            }
            previous_digit = Some(digit);
        }

        return seen_duplicate;*/

        let digits = int_to_digits(password);

        //rule: contains double
       /*
       algo:
        while n < len
        n
        |
        1234567

        compare with n+1, if equal than true,
        else inc n

        Important: no odd multiply of value allowed
        */



        let mut doubles = false;
        let mut count_doubles = 0;

        for n in 0..5 {
            //let mut cnt_digit = digits.iter().filter(|&n| *n == digit).count();
            if digits[n] == digits[n+1] {
                doubles = true;
                count_doubles = count_doubles +1;
            }
        }

        if doubles == false {
            return false;
        }

        //rule: never decrease
        let mut smaller_digit = 0;
        for digit in digits {
            if smaller_digit <= digit {
                smaller_digit = digit;
            }
            else {
                return false;
            }
        }
        return true;
    }


    pub fn run() {
        let range_from = 382345;
        let range_to = 843167;
        let mut valid_counter = 0;
        for i in range_from+1..range_to {
            if check_password(i) == true {
                valid_counter += 1;
            }
        }
        println!("{} valid passwords in range", valid_counter);
    }

    #[test]
    fn check_password_test() {
        assert_eq!(check_password(111111), true);
        assert_eq!(check_password(223450), false);
        assert_eq!(check_password(123789), false);
        assert_eq!(check_password(777999), true);
    }
}

#[allow(dead_code)]
pub mod part2 {

    //https://stackoverflow.com/questions/41536479/splitting-an-integer-into-individual-digits
    fn int_to_digits(n: i32) -> Vec<i32> {
        fn x_inner(n: i32, xs: &mut Vec<i32>) {
            if n >= 10 {
                x_inner(n / 10, xs);
            }
            xs.push(n % 10);
        }
        let mut xs = Vec::new();
        x_inner(n, &mut xs);
        xs
    }

    fn check_password(password: i32) -> bool {

        let digits = int_to_digits(password);
        let mut doubles = false;
        let mut count_doubles = 0;

        for n in 0..5 {
            if digits[n] == digits[n+1] {
                count_doubles = count_doubles +1;
            }
            else {
                if count_doubles == 1 {
                    doubles = true;
                }
                count_doubles = 0;
            }
        }

        if doubles == false &&  count_doubles != 1 {
            return false;
        }

        //rule: never decrease
        let mut smaller_digit = 0;
        for digit in digits {
            if smaller_digit <= digit {
                smaller_digit = digit;
            }
            else {
                return false;
            }
        }
        //println!("{}", password);
        return true;
    }


    pub fn run() {
        let range_from = 382345;
        let range_to = 843167;
        let mut valid_counter = 0;
        for i in range_from+1..range_to {
            if check_password(i) == true {
                valid_counter += 1;
            }
        }
        println!("{} valid passwords in range", valid_counter);
        //215 is to low
    }

    #[test]
    fn check_password_test() {
        assert_eq!(check_password(112233), true);
        assert_eq!(check_password(123444), false);
        assert_eq!(check_password(111122), true);
    }
}