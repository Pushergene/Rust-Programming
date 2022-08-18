use std::collections::HashMap;

/// https://leetcode.com/problems/reduce-array-size-to-the-half/
impl Solution {
    pub fn min_set_size(nums: Vec<i32>) -> i32 {
        let len_ns: usize = nums.len();
        let num_to_freqs: HashMap<i32, u32> = {
            let mut num_to_freqs: HashMap<i32,u32> = HashMap::with_capacity(len_ns);
            //let mut map: HashMap<i32, u32> = HashMap::with_capacity(len_ns);
            for num in nums{
                *num_to_freqs.entry(num).or_default() += 1;
            }
            num_to_freqs
        };
        let freqs: Vec<u32> = {
            let mut freqs: Vec<u32> = num_to_freqs.into_values().collect();
            freqs.sort();
            freqs
        };
        let mut cnt: i32 = 0;
        let mut len: u32 = 0;
        let len_ns: u32 = len_ns as u32;
        for freq in freqs.into_iter().rev(){
            len += freq;
            cnt += 1;
            if len >= len_ns / 2{
                return cnt;
            }
        }
        unreachable!();
    }
}
