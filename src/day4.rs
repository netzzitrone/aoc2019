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

        */
        let mut doubles = false;
        for n in 0..4 {
            if digits[n] == digits[n+1] {
                doubles = true;
                break;
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
       // let range_from = 111111;
       // let range_to = 111141;
        let mut valid_counter = 0;
        for i in range_from+1..range_to {
            if check_password(i) == true {
                valid_counter += 1;
                println!("{}", i);
            }

        }
        println!("{} valid passwords in range", valid_counter);
        //454 is wrong
    }

    #[test]
    fn check_password_test() {
        assert_eq!(check_password(111111), true);
        assert_eq!(check_password(223450), false);
        assert_eq!(check_password(123789), false);
    }
}