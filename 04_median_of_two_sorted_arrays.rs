impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        //make temp vec to merge both vecs
        let mut m_vec: Vec<i32> = vec![0; nums1.len() + nums2.len()];
        //used to iterate through temp vec
        let mut _m: usize = 0;
        //populate temp vec
        for _i in 0..nums1.len(){
            m_vec[_m] = nums1[_i];
            _m += 1;
        }
        for _i in 0..nums2.len(){
            m_vec[_m] = nums2[_i];
            _m += 1;
        } 
        //sort vec
        m_vec.sort();
        
        //decide if the len is even or odd to get median
        if m_vec.len() % 2 == 0 && m_vec.len() > 1 {
            let m_len = m_vec.len() / 2;
            return (m_vec[m_len] + m_vec[m_len - 1]) as f64 / 2.0
        } else {
            let m_len = (m_vec.len() - 1) / 2;
            return m_vec[m_len] as f64;
        }
        
    }

}
