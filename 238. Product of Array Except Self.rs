impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![1; nums.len()];

        let mut prefix = 1;
        for i in 0..nums.len() - 1{
            prefix *= nums[i];
            res[i + 1] *= prefix; 
        }

        let mut suffix = 1;
        for i in (0..nums.len() - 1).rev(){
            suffix *= nums[i + 1];
            res[i] *= suffix; 
        }
        return res;
    }
}