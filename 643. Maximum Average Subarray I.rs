use std::cmp;
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum : i32 = nums[0..k as usize].iter().sum();
        let mut max = sum;
        for i in k..nums.len() as i32{
            let new_sum = sum - nums[(i - k) as usize] + nums[i as usize];
            max = cmp::max(max, new_sum);
            sum = new_sum;
        }
        return max as f64 / k as f64;
    }
}