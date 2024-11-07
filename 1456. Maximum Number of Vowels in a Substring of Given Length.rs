use std::cmp;
impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let letters = ['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'];
        
        let s_slice = &s[0..(k as usize)];
        let s_filter = s_slice.chars().filter(|c| letters.contains(&c));
        let mut vowel_count = s_filter.count();
        let mut max_count: i32 = vowel_count as i32;

        let mut start_iter = s.chars();
        let mut end_iter = s.chars().skip(k as usize - 1);

        let mut start_char = start_iter.next().unwrap();
        let mut end_char = end_iter.next().unwrap();
        /*for i in 0..k{
            end_char = end_iter.next().unwrap();
        }*/
        
        for i in 1_usize..(s.len() + 1 - k as usize) {
            if letters.contains(&start_char){
                vowel_count -= 1;
            }
            start_char = start_iter.next().unwrap();
            end_char = end_iter.next().unwrap();
            if letters.contains(&end_char){
                vowel_count +=1;
            }
            max_count = cmp::max(max_count, vowel_count as i32);
        }
        return max_count;
    }
}