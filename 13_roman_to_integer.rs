//avoiding hashmap
/*
I = 73
V = 86

X = 130
L = 76

C = 67
D = 68

M = 77 
*/
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        //store roman numerals to compare
        let roman_vec: Vec<char> = vec!['0','I','V','X','L','C','D','M'];
        //convert string to char vec //not touching parameters
        let s_vec: Vec<char> = s.chars().collect();
        let mut roman_to_int: i32 = 0; 

        //iterate through string
        let mut _i: usize = 0; 
        let mut _j: usize = 0;

        //find j appointment based on first char of s
        while _j < roman_vec.len() && s_vec[_i] != roman_vec[_j] {
            _j += 1;
        }

        //each odd element can repeat (not implemented, just an observation)
        //reaching an odd element means we will check -> 
        //if the next s.char equates the next two elements (if not out of bounds)
        //run cases here and get roman value dependent on element pos
        //if pos = 7, then 1 * 5 * 2 * 5 * 2 * 5 * 2, meaning recurse
        while _i < s_vec.len() {
            //checking for divisible of 4
            if ((_i + 1) < s_vec.len() && (_j + 1) < roman_vec.len()) && s_vec[_i + 1] == roman_vec[_j + 1] {
                roman_to_int += (sum_my_roman(_j + 1) - sum_my_roman(_j)); 
                _i += 1;
            }
            //checking for divisible of 9 
            else if ((_i + 1) < s_vec.len() && (_j + 2) < roman_vec.len()) && s_vec[_i + 1] == roman_vec[_j + 2] {
                roman_to_int += (sum_my_roman(_j + 2) - sum_my_roman(_j));
                _i += 1;
            }
            else {
                //add initial
                roman_to_int += sum_my_roman(_j); 

                //check for repeating
                let c_test: char = s_vec[_i];
                while _i + 1 < s_vec.len() && c_test == s_vec[_i + 1]  {
                    roman_to_int += sum_my_roman(_j);
                    _i += 1;
                }
                    
                 
            }
            
            _i += 1;
            
            if _i < s_vec.len() {
                _j = 0;
                while _j < roman_vec.len() && s_vec[_i] != roman_vec[_j] {
                    _j += 1;
                }
            }
        }

        return roman_to_int
    }
}

pub fn sum_my_roman(counter: usize) -> i32 {
    let mut value: i32 = 1;

    for n in 0..(counter - 1) {
        if (n + 2) % 2 == 0 {
            value *= 5;
        } else {
            value *= 2;
        }
    }

    return value
}

