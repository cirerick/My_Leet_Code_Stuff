impl Solution {
    pub fn reverse(x: i32) -> i32 { 
        let (sign, mut temp_x): (bool, i32) = match x {
            n if n < 0 && n != i32::MIN => (true, (x * (-1))), //impossible to reverse min of i32
            n if n > 0 => (false, x),
            _ => (false, 0), 
        };

        if Solution::get_amount_digits(temp_x) >= 10 { //check for amount of digits, seems naive 
            if !Solution::overflow_check(temp_x, i32::MAX / 1000000000, 100000000){
                return 0
            }
        }

        let mut rev: i32 = 0;

        while temp_x > 0 {
                rev = (rev * 10) + (temp_x % 10); //"adding" ones place
                temp_x /= 10; //"removing" ones place
        }

        //apply sign
        if sign {
            return rev * (-1)
        } else {
            return rev
        }
    }

    fn overflow_check(n: i32, i32_max: i32, placement: i32) -> bool  {
        if n > 0 { 
            match n % 10 {
                x if x == i32_max % 10 => Solution::overflow_check(n / 10, i32::MAX / placement, placement / 10), 
                x if x < i32_max % 10 => return true,
                x if x > i32_max % 10 => return false,
                _=> return false,
            }
        } else {
            return false //impossible to reverse max of i32
        }
    }

    fn get_amount_digits(n: i32) -> u8 {
        let mut counter: u8 = 0;
        let mut temp_n: i32 = n;
        while temp_n > 0 {
            temp_n /= 10;
            counter += 1;
        }

        return counter;
    }

}
