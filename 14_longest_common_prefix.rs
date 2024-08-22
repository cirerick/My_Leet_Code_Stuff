impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        //store the longest common prefix by character
        let mut lcp: Vec<char> = strs[0].chars().collect(); //lcp now gains ownership of these chars specifically

        for _i in 1..strs.len() {
            let c_str: String; //str we are checking
            //check if prefix is shorter than string
            if lcp.len() > strs[_i].len() {
                //swap
                c_str = lcp.iter().collect(); 
                lcp = strs[_i].chars().collect(); 
            } else {
                c_str = strs[_i].clone();
            } //we could have maybe sliced here as lcp = &str[0..strs[_i].len]
            

            //check charcters for prefix and string
            for _j in 0..lcp.len() {
                if lcp[_j] != c_str.chars().nth(_j).unwrap() {
                    for blankity in 0..lcp.len() - _j {
                        lcp.pop();
                    }
                    break; 
                }
               
            }

            if lcp.is_empty() {
                break;
            }

        }
        
        return lcp.iter().collect()
    }
}

