impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        //There are constraints, but just in case
        if num_rows <= 1 || s.len() <= num_rows as usize{
            return s;
        }

        let zig: usize = num_rows as usize; //zig is cooler
        let zag: usize = zig - 2; // length of diagonal trajectory   
        let mut temp_vec: Vec<char> = Vec::with_capacity(s.len()); 

        //populate the first row
        let mut _i: usize = 0;
        while _i < s.len() {
            temp_vec.push(s.chars().nth(_i).unwrap());
            _i += zig + zag; 
        }
        
        //populate middle portions   
        for _j in 1..zig - 1 {
            println!("_J: {}", _j);
            let mut parity_counter: usize = 0; //determine if iteration is even or odd
            _i = _j;
            while _i < s.len() {
                if (parity_counter + 2) % 2 == 0 {
                    temp_vec.push(s.chars().nth(_i).unwrap());
                    _i += ((zig + zag) - (_j * 2));
                    parity_counter += 1;
                } else {
                    temp_vec.push(s.chars().nth(_i).unwrap());
                    _i += (2 * _j);
                    parity_counter += 1;
                }
            }
        }

        //populate last row
        _i = zig - 1;
        while _i < s.len() {
            temp_vec.push(s.chars().nth(_i).unwrap());
            println!("_i: {}", _i);
            _i += zig + zag; 
        }

        return temp_vec.into_iter().collect()
    }
}
