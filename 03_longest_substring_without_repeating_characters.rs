impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        //check to make sure there is something inside the string first
        if s.is_empty() {
            return 0
        }
        //decalre array of char used to compare the string for repeating letters
        let mut checker_vec: Vec<char> = Vec::with_capacity(s.len());
        //keep track of substring lengths
        let mut counter: i32 = 0;
        //store max len of a substring
        let mut max: i32 = 0;

        //s.collect() would allow to populate a vec, but I want to populate it myself
        let mut s_as_char_vec: Vec<char> = Vec::with_capacity(s.len()); //avoid touching passed s
        for _c in s.chars() { //populate s_as_char_vec with passed s
            s_as_char_vec.push(_c);
        }

        let s_len: usize = s_as_char_vec.len();

        //iterate through vec of s forward
        for _i in 0..s_as_char_vec.len() {
            //first step
            if counter > max {
                max = counter;
            }
           
            checker_vec.push(s_as_char_vec[_i]);
            counter += 1;

            //iterate through checker_vec
            for _j in 0..(checker_vec.len() - 1) {
                if !checker_vec.is_empty() { //mitigate checking an empty vec
                    if checker_vec[_j] == s_as_char_vec[_i] {
                        checker_vec.clear();
                        counter = 0;
                    }
                }
            }

            //we clear the vec above
            if checker_vec.is_empty() {
                let mut _e: usize = _i;
                //regain chars from after first instance of repeated char
                while s_as_char_vec[_e - 1] != s_as_char_vec[_i] && (_e - 1) >= 0{
                    checker_vec.push(s_as_char_vec[_e - 1]);
                    counter += 1;
                    _e -= 1;
                }  
                //collect current char
                checker_vec.push(s_as_char_vec[_i]); 
                counter += 1;
            }

        }

        //band-aid fix for when the last element is not counted upon forloop exit without incrementing counter
        if (checker_vec.len() as i32) > max {
            max += 1;
        }


        return max;
    }
}
