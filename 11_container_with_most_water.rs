//brute force
//time exceeded
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area: i32 = 0;
        let mut _i: usize = 0;
        let mut _j: usize = height.len() - 1;

        while _i < _j {
            let area: i32 = Solution::get_area(height[_i], height[_j], (_j - _i) as i32);
            
            if max_area < area {
                max_area = area;
            }

            if height[_i] < height[_j] {
                _i += 1;
            } else {
                _j -= 1;
            }
        }

        return max_area
    }

    //xh = _i height and yh = _j height // or latter height
    fn get_area(mut xh: i32, mut yh: i32, interval: i32) -> i32 {
        if xh < yh {
            yh = xh
        } else {
            xh = yh; 
        }

        return xh * interval

    }
}