use std::collections::HashSet;
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowel_set = HashSet::from([b'a', b'e', b'i', b'o', b'u', b'A', b'E', b'I', b'O', b'U']);
        
        let mut new_s = s.into_bytes();
        
        let mut start = 0;
        let mut end = new_s.len() - 1;
        let mut moved = false;
        
        while start < end{
            if vowel_set.contains(&new_s[start]) && vowel_set.contains(&new_s[end]){
                new_s.swap(start, end);
                start +=1;
                end -= 1;
                moved = true;
            }

            if !vowel_set.contains(&new_s[start]) && moved == false{
                start += 1;
            }
            println!("{} {}", start, end);
            if !vowel_set.contains(&new_s[end]) && moved == false{
                end -= 1;
            }
            moved = false;
        }

        return String::from_utf8(new_s).unwrap();
    }
}