//constraints: num cannot be less than 1 or more than 3999
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        //hard store 
        let roman_sym: Vec<char> = vec!['0', 'I','V','X','L','C','D', 'M'];

        let mut i_to_char: Vec<char> = Vec::new();

        //find amount of placements
        let digit_amount: usize = greatest_placement(num.abs() as usize);
        let window: usize = roman_sym.len() - (2 * digit_amount);
        for _i in 1..=digit_amount {
            let digit: usize = (num.abs() as usize) / positive_power(10, digit_amount - _i) % 10; 
            match digit {
                4 => {
                    i_to_char.push(roman_sym[((8 - (2*_i)) - (window)) + 1]); //wrote 8 instead of roman_sym.len()
                    i_to_char.push(roman_sym[((8 - (2*_i)) - (window)) + 2]);
                },
                5 => {
                    i_to_char.push(roman_sym[((8 - (2*_i)) - (window)) + 2]);
                },
                9 => {
                    i_to_char.push(roman_sym[((8 - (2*_i)) - (window)) + 1]);
                    i_to_char.push(roman_sym[((8 - (2*_i)) - (window)) + 3]);
                }, 
                _ => {
                    if digit < 5 {
                        for _j in 0..digit{
                            i_to_char.push(roman_sym[((8 - (2*_i)) - (window)) + 1]);
                        }
                    } else {
                        i_to_char.push(roman_sym[((8 - (2*_i)) - (window)) + 2]);
                        for _j in 0..(digit - 5){
                            i_to_char.push(roman_sym[((8 - (2*_i)) - (window)) + 1]);
                        }
                    }
                }
            }
        }
        return i_to_char.into_iter().collect()
    }

}

pub fn positive_power(base: usize, power: usize) -> usize {
    if power == 0 {
        return 1
    }

    let mut result: usize = base;
    for _i in 1..power {
        result = result * base;
    }
    
    return result
}

fn greatest_placement(x: usize) -> usize {
    let mut temp_x: usize = x;
    let mut counter: usize = 0;
    while temp_x > 0 {
        temp_x /= 10;
        counter += 1;
    }

    return counter
}
