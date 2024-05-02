impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        //negative numbers cannot be palindromes
        if x < 0 {
            return false
        }

        let mut rev_x: i32 = 0;
        let mut temp_x: i32 = x; //will not touch parameters
        
        while temp_x > 0 {
            if rev_x.checked_mul(10).is_some() {
                rev_x = (rev_x * 10) + (temp_x % 10);
                temp_x /= 10;
            } else {
                return false //an overflow means it could not be a palindrome
            }
        }

        if rev_x == x {
            return true
        } else {
            return false
        }
    }
}
