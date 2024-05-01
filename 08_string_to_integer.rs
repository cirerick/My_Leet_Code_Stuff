/* According to the Problem.

Examples of the solution being negative:
"     -1" -> -1
"-87d48" -> -87

When '-' should return 0:
" - 1" -> There is a space between the 1 and '-'
"d -256" -> The String starts with a char
"--42" -> Must lead with white spaces
"0-01" -> No leading zeros before '-'

*/

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        //fallback
        if s.len() <= 0 {
            return 0
        }

        //vec method //s.trim_start() could have been used then proceed with nth
        let vec_stou8: Vec<u8> = s.as_bytes().to_vec();
        let mut vec_atou8: Vec<u8> = Vec::new();

        //trim white spaces
        let mut _i: usize = 0;
      
        while _i < vec_stou8.len() {
            if vec_stou8[_i] != 32 {
                vec_atou8.push(vec_stou8[_i]);
                break 
            }
            _i += 1;
        }

        //if s is all white spaces, exit
        if vec_atou8.is_empty() {
            return 0
        } 

        _i += 1;

        //iterate until out of range of real digits
        while  _i < vec_stou8.len() {
            if vec_stou8[_i] >= 48 && vec_stou8[_i] <= 57 {
                vec_atou8.push(vec_stou8[_i]); 
            } else {
                break
            }
            _i += 1;
        }

        //revert to string and error handle depending on overflow or underflow
        let mut yikes: i32 = match String::from_utf8(vec_atou8).unwrap().parse() {
            Ok(whatever_gets_parsed_and_passed) => whatever_gets_parsed_and_passed, //longer the name, the better
            Err(e) => match e.kind() {
                std::num::IntErrorKind::PosOverflow => return i32::MAX,
                std::num::IntErrorKind::NegOverflow => return i32::MIN,
                _ => return 0,
            }, 
        };
        
        return yikes
    }
}
