impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        //no point in moving forward when there is only one char
        if s.len() == 1 {
            return s
        }
        //store positions of longest palindrome
        let mut interval: (usize, usize) = (0,0);
        let mut max_dist: usize = 0;  

        //determine if odd or even, then set midpoint
        let midpoint: usize = match (s.len() + 2) % 2{
            0 => (s.len() / 2) - 1,
            1 => s.len() / 2,
            _ => 0,
        };

        let mut checkpoint: usize = midpoint; //will be moved to be used as origin for checks
        
        //to check each char with readability

        //odd palindrome check
        //left
        println!("=====Checking left side:======");
        //if the possible max has been reached from the previous iteration, stop.
        while checkpoint > 0 && max_dist < (checkpoint * 2) {
            println!("CHKPT: {}", checkpoint);
            for _i in 1..checkpoint + 1 {
                if  s.chars().nth(checkpoint - _i).unwrap() == s.chars().nth(checkpoint + _i).unwrap() {
                    println!("Elem: {}|{}",checkpoint - _i, s.chars().nth(checkpoint - _i).unwrap());
                    println!("Elem: {}|{}",checkpoint + _i, s.chars().nth(checkpoint + _i).unwrap());
                    if ((checkpoint + _i) - (checkpoint - _i)) > max_dist {
                        interval = (checkpoint - _i, checkpoint + _i);
                        max_dist = interval.1 - interval.0;
                        println!("\nINT: {:?}", interval);
                        println!("MAX: {}\n", max_dist);  
                    }  
                } else {
                    break;
                } 
            }
            checkpoint -= 1;
        }

        
        //right
        println!("\n=====Checking right side:=====");
        checkpoint = midpoint + 1; //adding one as we had already checked midpoint to the right
        while checkpoint < s.len() && max_dist < ((s.len() - checkpoint - 1) * 2) {
            println!("CHKPT: {}", checkpoint);
            for _i in 1..s.len() - checkpoint{
                if s.chars().nth(checkpoint - _i).unwrap() == s.chars().nth(checkpoint + _i).unwrap(){
                    println!("Elem: {}|{}",checkpoint - _i, s.chars().nth(checkpoint - _i).unwrap());
                    println!("Elem: {}|{}",checkpoint + _i, s.chars().nth(checkpoint + _i).unwrap());
                    if ((checkpoint + _i) - (checkpoint - _i)) > max_dist {
                        interval = (checkpoint - _i, checkpoint + _i);
                        max_dist = interval.1 - interval.0;
                        println!("\nINT: {:?}", interval);
                        println!("MAX: {}\n", max_dist);  
                    }
                } else {
                    break;
                }
            }
            checkpoint += 1;
        }

        

        checkpoint = midpoint;
        
        //even check
        println!("\nENETERING EVEN CHECK!\n"); 
        //left
        println!("=====Checking left side:======");
        while checkpoint > 0 && max_dist < (checkpoint * 2) {
            println!("CHKPT: {}", checkpoint);
            for _i in 0..checkpoint {
                if s.chars().nth(checkpoint + _i).unwrap() == s.chars().nth(checkpoint - _i - 1).unwrap(){
                    println!("Elem: {}|{}",checkpoint + _i, s.chars().nth(checkpoint + _i).unwrap());
                    println!("Elem: {}|{}",checkpoint - _i - 1, s.chars().nth(checkpoint - _i - 1).unwrap());
                    if ((checkpoint + _i) - (checkpoint - _i - 1)) > max_dist {
                        interval = (checkpoint - _i - 1, checkpoint + _i);
                        max_dist = interval.1 - interval.0;
                        println!("\nINT: {:?}", interval);
                        println!("MAX: {}\n", max_dist);  
                    }
                }else{
                    break;
                }
            }
            checkpoint -= 1;
        }


        checkpoint = midpoint + 1;

        //right 
        println!("\n=====Checking right side:=====");
        while checkpoint < s.len() && max_dist < (s.len() - checkpoint) * 2 {
            println!("CHKPT: {}", checkpoint);
            for _i in 0..s.len() - checkpoint {
                if s.chars().nth(checkpoint + _i).unwrap() == s.chars().nth(checkpoint - _i - 1).unwrap() {
                    println!("Elem: {}|{}",checkpoint + _i, s.chars().nth(checkpoint + _i).unwrap());
                    println!("Elem: {}|{}",checkpoint - _i - 1, s.chars().nth(checkpoint - _i - 1).unwrap());
                    if ((checkpoint + _i) - (checkpoint - _i - 1)) > max_dist {
                        interval = (checkpoint - _i - 1, checkpoint + _i);
                        max_dist = interval.1 - interval.0; 
                        println!("\nINT: {:?}", interval);
                        println!("MAX: {}\n", max_dist);  
                    }
                } else {
                    break;
                }
            } 
            checkpoint += 1;
        }

        //return slice of longest
        if max_dist > 0 {
            return (&s[interval.0..interval.1 + 1]).to_string()
        } else {
            return (&s[0..1]).to_string()
        }
    }
}