impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        //store positions of longest palindrome
        let mut interval: (usize, usize) = (0,0);
        let mut max_dist: usize = 0;  

        //determine if odd or even, then set midpoint
        let midpoint: usize = match s.len() % 2{
            0 => (s.len() / 2) - 1,
            1 if s.len() == 1 => 1,
            1 => s.len() / 2,
            _ => 0,
        };

        let mut checkpoint: usize = midpoint; //will be moved to be used as origin for checks
        
        //to check each char with readability

        //odd palindrome check
        //left
        println!("Checking left side");
        while checkpoint > 0 {
            println!("MID: {}", checkpoint);
            for _i in 1..checkpoint + 1 {
                if (checkpoint - _i) >= 0 {
                    println!("Elem: {}|{}",checkpoint - _i, s.chars().nth(checkpoint - _i).unwrap());
                } 
                if (checkpoint + _i) <= s.len() - 1{
                    println!("Elem: {}|{}",checkpoint + _i, s.chars().nth(checkpoint + _i).unwrap());
                } 
            }
            checkpoint -= 1;
        }
        //right
        println!("\nchecking right side");
        checkpoint = midpoint + 1; //adding one as we had already checked midpoint to the right
        while checkpoint < s.len() - 1 {
            println!("MID: {}", checkpoint);
            for _i in 1..s.len() - checkpoint{
                println!("I: {}", _i); 
                if (checkpoint - _i) >= 0 {
                    println!("Elem: {}|{}",checkpoint - _i, s.chars().nth(checkpoint - _i).unwrap());
                }
                if (checkpoint + _i) <= s.len() - 1{
                    println!("Elem: {}|{}",checkpoint + _i, s.chars().nth(checkpoint + _i).unwrap());
                }
            }
            checkpoint += 1;
        }

        //even check
    



        //return slice of longest
        return "nothing".to_string()
    }
}
