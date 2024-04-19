impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result_vec: Vec<i32> = Vec::with_capacity(2);
        for _i in 0..nums.len() {
            for _j in (_i + 1)..nums.len() {
                if (nums[_i] + nums[_j]) == target {
                    result_vec.push(_i as i32);
                    result_vec.push(_j as i32);
                    break;
                }
            }
        }

        return result_vec        
    }

}