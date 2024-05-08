impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s_arr: &[u8] = s.as_bytes();
        let p_arr: &[u8] = p.as_bytes();
        let mut dp_table: Vec<Vec<bool>> = vec![vec![false; p_arr.len() + 1]; s_arr.len() + 1];
        dp_table[0][0] = true;

        //initialize the first row
        for _j in 1..=p_arr.len() {
            if p_arr[_j - 1] == 42 {
                dp_table[0][_j] = dp_table[0][_j - 2];
            }
        }

        //fill the DP table
        for _i in 1..=s.len() {
            for _j in 1..=p_arr.len() {
                if p_arr[_j - 1] == 42 {
                    dp_table[_i][_j] = dp_table[_i][_j - 2] 
                    || (dp_table[_i - 1][_j] && (s_arr[_i - 1] == p_arr[_j - 2] 
                    || p_arr[_j - 2] == 46));
                } else {
                    dp_table[_i][_j] = dp_table[_i - 1][_j - 1] && (s_arr[_i - 1] == p_arr[_j - 1] 
                    || p_arr[_j - 1] == 46);
                }
            }
        }

        dp_table[s_arr.len()][p_arr.len()]
    }
}
